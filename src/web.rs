use std::error::Error;

use rustful::{Server, Context, Response, TreeRouter};

fn say_hello(context: Context, response: Response) {
    let person = match context.variables.get("person") {
        Some(name) => name,
        None => "stranger".into()
    };

    response.send(format!("Hello, {}!", person));
}

pub fn server_start(port:u16) {
  let server_result = Server {
    host: port.into(),

    handlers: insert_routes!{
      TreeRouter::new() => {
        Get: say_hello,

        ":person" => Get: say_hello
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
