use std::fs;
use std::io;
use std::io::prelude::*;
use std::path::Path;
use std::process::{Command, Stdio};
use std::os::unix::io::{FromRawFd};

use conf::Conf;
use seed::Seed;

#[inline(always)]
fn setup_env(conf:&Conf, buf:&Vec<u8>) {
  let _ = fs::remove_file(&conf.input_path);

  let mut f = fs::File::create(&conf.input_path).unwrap();
  f.write_all(buf).unwrap();
}

#[inline(always)]
fn clear_env(conf:&Conf) {
  let _ = fs::remove_file(&conf.input_path);
}

pub fn exec(conf:&Conf, buf:&Vec<u8>) {
  setup_env(&conf, &buf);

  let (prog, args) = match conf.args.split_first() {
    Some((prog, args)) => (prog, args),
    None => panic!("Too few command line arguments")
  };

  let mut child = unsafe {
    Command::new(prog)
      .args(args)
      .stdin(Stdio::from_raw_fd(conf.stdin_fd))
      .stdout(Stdio::null())
      .stderr(Stdio::null())
      .spawn()
      .expect("failed to execute child")
  };

  let status = child.wait().expect("Child Wait");

  clear_env(&conf);
}
