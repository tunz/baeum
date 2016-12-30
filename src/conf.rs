use std::fs;
use std::io::prelude::*;
use std::path::Path;
use std::process;
use std::os::unix::io::{RawFd, IntoRawFd};
use std::env;

pub struct Conf {
  pub args: Vec<String>,
  pub input_path: String,
  pub output_dir: String,
  pub stdin_fd: RawFd,
  pub timeout: u64,
}

static mut CRASH_COUNT: u32 = 0;

impl Conf {
  pub fn new(args:Vec<&str>, output_dir:&str, t:u64, input_path: &str) -> Conf {
    if Path::new(&output_dir).exists() {
      println!("Error: {} already exists", output_dir);
      process::exit(1)
    }

    fs::create_dir(&output_dir).unwrap();
    fs::create_dir(format!("{}/queue", output_dir)).unwrap();
    fs::create_dir(format!("{}/crash", output_dir)).unwrap();
    let stdin_fd = fs::File::create(format!("{}/.stdin", output_dir)).unwrap().into_raw_fd();

    let input_path = match args.iter().find(|&&s| s == "@@") {
                       Some(_) => String::from(input_path),
                       None => format!("{}/.stdin", output_dir)
                     };
    let mut args:Vec<String> =  args.iter().map(|&s| if s == "@@" { input_path.clone() }
                                                     else { String::from(s) }).collect();
    let qemu_path = match Path::new(&env::args().nth(0).unwrap()).parent() {
                      Some(p) => format!("{}/qemu-trace-coverage", p.to_str().unwrap()),
                      None => String::from("/qemu-trace-coverage")
                    };
    args.insert(0, qemu_path);

    Conf {
      args: args,
      output_dir: String::from(output_dir),
      input_path: input_path,
      stdin_fd: stdin_fd,
      timeout: t,
    }
  }

  pub fn new_without_filename(args:Vec<&str>, output_dir:&str, t:u64) -> Conf {
    let filepath = format!("{}/.input", output_dir);
    Conf::new(args, output_dir, t, &filepath)
  }

  pub fn save_crash(&self, buf:&Vec<u8>) {
    let path = unsafe {
      CRASH_COUNT = CRASH_COUNT + 1;
      format!("{}/crash/tc-{}", self.output_dir, CRASH_COUNT)
    };
    let mut f = fs::File::create(&path).unwrap();
    f.write_all(buf).unwrap();
  }
}

