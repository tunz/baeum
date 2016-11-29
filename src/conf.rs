use std::fs;
use std::io;
use std::io::prelude::*;
use std::path::Path;
use std::process;

pub struct Conf {
  pub args: Vec<String>,
  pub input_path: String,
  pub output_dir: String,
}

impl Conf {
  pub fn new(args:Vec<&str>, output_dir:&str, input_path: &str) -> Conf {
    if Path::new(&output_dir).exists() {
      println!("Error: {} already exists", output_dir);
      process::exit(1)
    }

    fs::create_dir(&output_dir).unwrap();
    fs::create_dir(format!("{}/queue", output_dir)).unwrap();
    fs::create_dir(format!("{}/crash", output_dir)).unwrap();

    Conf {
      args: args.iter().map(|&s|
                              if s == "@@" { String::from(input_path) }
                              else { String::from(s) }
                            ).collect(),
      output_dir: String::from(output_dir),
      input_path: String::from(input_path)
    }
  }

  pub fn new_without_filename(args:Vec<&str>, output_dir:&str) -> Conf {
    let filepath = format!("{}/.input", output_dir);
    Conf::new(args, output_dir, &filepath)
  }
}

