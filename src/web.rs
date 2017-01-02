use std::error::Error;
use std::io::{self, Read};
use std::fs::File;
use std::path::Path;
use std::sync::{Arc, RwLock};

use rustful::{Server, Context, Response, Handler, TreeRouter, StatusCode};
use rustful::file::check_path;

use conf;

enum Api {
  Hello { page: String },
  Info { log: Arc<RwLock<conf::Log>> },
  File
}

fn read_string<P: AsRef<Path>>(path: P) -> io::Result<String> {
    let mut string = String::new();
    File::open(path).and_then(|mut f| f.read_to_string(&mut string)).map(|_| string)
}

impl Handler for Api {
  fn handle_request(&self, context: Context, mut response: Response) {
    match *self {
      Api::Hello { ref page } => {
        response.send(page.as_str());
      },
      Api::Info { ref log } => {
        let (seed_count, crash_count) = { let log = log.read().unwrap(); (log.seed_count, log.crash_count) };
        response.send(format!("Seed Count: {}<br/>Crash Count: {}", seed_count, crash_count));
      },
      Api::File => {
        if let Some(file) = context.variables.get("file") {
          let file_path = Path::new(file.as_ref());

          if check_path(file_path).is_ok() {
            let path = Path::new("ui").join(file_path);
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

pub fn server_start(port:u16, path_base:String, log:Arc<RwLock<conf::Log>>) {
  let page = read_string(format!("{}/ui/index.html", path_base)).unwrap();
  let server_result = Server {
    host: port.into(),

    handlers: insert_routes!{
      TreeRouter::new() => {
        Get: Api::Hello { page: page.clone() },
        "info/*info" => Get: Api::Info { log: log.clone() },
        "res/*file" => Get: Api::File
      }
    },

    ..Server::default()
  }.run();

  println!("Web Server: http://0.0.0.0:{}", port);

  match server_result {
    Ok(_server) => {},
    Err(e) => error!("could not start server: {}", e.description())
  }
}
