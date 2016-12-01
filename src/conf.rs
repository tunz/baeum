use std::fs;
use std::io::prelude::*;
use std::path::Path;
use std::process;
use std::os::unix::io::{RawFd, IntoRawFd};

pub struct Conf {
  pub args: Vec<String>,
  pub input_path: String,
  pub output_dir: String,
  pub stdin_fd: RawFd,
}

static mut crash_count: u32 = 0;

impl Conf {
  pub fn new(args:Vec<&str>, output_dir:&str, input_path: &str) -> Conf {
    if Path::new(&output_dir).exists() {
      println!("Error: {} already exists", output_dir);
      process::exit(1)
    }

    fs::create_dir(&output_dir).unwrap();
    fs::create_dir(format!("{}/queue", output_dir)).unwrap();
    fs::create_dir(format!("{}/crash", output_dir)).unwrap();
    let stdin_fd = fs::File::create(format!("{}/.stdin", output_dir)).unwrap().into_raw_fd();

    Conf {
      args: args.iter().map(|&s|
                              if s == "@@" { String::from(input_path) } // XXX: If target is STDIN?
                              else { String::from(s) }
                            ).collect(),
      output_dir: String::from(output_dir),
      input_path: String::from(input_path),
      stdin_fd: stdin_fd
    }
  }

  pub fn new_without_filename(args:Vec<&str>, output_dir:&str) -> Conf {
    let filepath = format!("{}/.input", output_dir);
    Conf::new(args, output_dir, &filepath)
  }

  pub fn save_crash(&self, buf:&Vec<u8>) {
    let path = unsafe {
      crash_count = crash_count + 1;
      format!("{}/crash/tc-{}", self.output_dir, crash_count)
    };
    let mut f = fs::File::create(&path).unwrap();
    f.write_all(buf).unwrap();
  }
}

