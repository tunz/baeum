use std::error::Error;
use std::io::{self, Read};
use std::fs::File;
use std::path::{Path, PathBuf};
use std::sync::{Arc, RwLock};

use rustc_serialize::json;
use rustful::{Server, Context, Response, Handler, TreeRouter, StatusCode};
use rustful::header::ContentType;
use rustful::file::check_path;

use stat;

// XXX: Use Rocket instaed of Rustful when Rust stable can run Rocket

#[derive(RustcDecodable, RustcEncodable)]
pub struct IdValue {
    id: String,
    value: String,
}

enum Api {
    Hello { page: String },
    Info { log: Arc<RwLock<stat::Log>> },
    Plot { log: Arc<RwLock<stat::Log>> },
    File { path_base: PathBuf },
}

fn read_string<P: AsRef<Path>>(path: P) -> io::Result<String> {
    let mut string = String::new();
    File::open(path).and_then(|mut f| f.read_to_string(&mut string)).map(|_| string)
}

fn sec_to_timef(secs: u64) -> String {
    let day = secs / 86400;
    let hour = (secs / 3600) % 24;
    let min = (secs / 60) % 60;
    let sec = secs % 60;
    format!("{} days, {:02}:{:02}:{:02}", day, hour, min, sec)
}

impl Handler for Api {
    fn handle_request(&self, context: Context, mut response: Response) {
        match *self {
            Api::Hello { ref page } => {
                response.send(page.as_str());
            }
            Api::Info { ref log } => {
                let (info, start_time) = {
                    let log = log.read().unwrap();
                    ((*log).info.clone(), (*log).data.start_time)
                };
                let t = start_time.elapsed().unwrap().as_secs();
                let execspeed = if t == 0 { 0 } else { info.exec_count / t };
                response.headers_mut()
                    .set(ContentType(content_type!(Application / Json; Charset = Utf8)));
                let object = vec![IdValue {
                                      id: "seed_count".to_string(),
                                      value: info.seed_count.to_string(),
                                  },
                                  IdValue {
                                      id: "crash_count".to_string(),
                                      value: info.crash_count.to_string(),
                                  },
                                  IdValue {
                                      id: "uniq_crash_count".to_string(),
                                      value: info.uniq_crash_count.to_string(),
                                  },
                                  IdValue {
                                      id: "total_node".to_string(),
                                      value: info.total_node.to_string(),
                                  },
                                  IdValue {
                                      id: "time".to_string(),
                                      value: sec_to_timef(t),
                                  },
                                  IdValue {
                                      id: "execspeed".to_string(),
                                      value: execspeed.to_string(),
                                  }];
                response.send(json::encode(&object).unwrap());
            }
            Api::Plot { ref log } => {
                let idx = match context.variables.get("idx") {
                    Some(i) => i.parse::<usize>().ok(),
                    None => Some(0),
                };
                let mut ret_infos: Vec<stat::LogInfo> = vec![];
                if let Some(idx) = idx {
                    let log = log.read().unwrap();
                    let ref infos = log.data.infos;
                    if (idx as usize) < infos.len() {
                        ret_infos.extend_from_slice(&infos[idx..]);
                    }
                };
                response.send(json::encode(&ret_infos).unwrap());
            }
            Api::File { ref path_base } => {
                if let Some(file) = context.variables.get("file") {
                    let file_path = Path::new(file.as_ref());

                    if check_path(file_path).is_ok() {
                        let path = Path::new(path_base).join(Path::new("ui").join(file_path));
                        let res = response.send_file(path)
                            .or_else(|e| e.send_not_found("the file was not found"))
                            .or_else(|e| e.ignore_send_error());

                        if let Err((error, mut response)) = res {
                            error!("filaed to open'{}': {}", file, error);
                            response.set_status(StatusCode::InternalServerError);
                        }
                    } else {
                        response.set_status(StatusCode::Forbidden);
                    }
                } else {
                    response.set_status(StatusCode::Forbidden);
                }
            }
        }
    }
}

pub fn server_start(port: u16, path_base: PathBuf, log: Arc<RwLock<stat::Log>>) {
    let page = read_string(path_base.join("ui/index.html")).unwrap();
    let server_result = Server {
            host: port.into(),

            handlers: insert_routes!{
                TreeRouter::new() => {
                    Get: Api::Hello { page: page.clone() },
                    "info/*info" => Get: Api::Info { log: log.clone() },
                    "plot/*idx" => Get: Api::Plot { log: log.clone() },
                    "res/*file" => Get: Api::File { path_base: path_base.clone() },
                }
            },

            ..Server::default()
        }
        .run();

    println!("Web Server: http://0.0.0.0:{}", port);

    match server_result {
        Ok(_server) => {}
        Err(e) => error!("could not start server: {}", e.description()),
    }
}
