use std::error::Error;
use std::io::{self, Read};
use std::fs::File;
use std::path::Path;
use std::sync::{Arc, RwLock};

use rustful;
use rustful::{Server, Context, Response, TreeRouter};

use conf;

fn read_string<P: AsRef<Path>>(path: P) -> io::Result<String> {
    let mut string = String::new();
    File::open(path).and_then(|mut f| f.read_to_string(&mut string)).map(|_| string)
}

fn say_hello(context: Context, response: Response, log:&Arc<RwLock<conf::Log>>) {
    let (seed_count, crash_count) = { let log = log.read().unwrap(); (log.seed_count, log.crash_count) };

    response.send(format!("Seed Count: {}<br/>Crash Count: {}", seed_count, crash_count));
}

struct Handler(fn(Context, Response, &Arc<RwLock<conf::Log>>), Arc<RwLock<conf::Log>>);

impl rustful::Handler for Handler {
    fn handle_request(&self, context: Context, response: Response) {
        self.0(context, response, &self.1);
    }
}

pub fn server_start(port:u16, path_base:String, log:Arc<RwLock<conf::Log>>) {
  // let page = read_string(format!("{}/ui/index.html", path_base)).unwrap();
  let server_result = Server {
    host: port.into(),

    handlers: insert_routes!{
      TreeRouter::new() => {
        Get: Handler(say_hello, log.clone())
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
