use std::fs;
use std::io::prelude::*;
use std::process::{Command, Stdio};
use std::os::unix::io::{FromRawFd};
use std::time::Duration;

use wait_timeout::ChildExt;

use conf::Conf;

enum ExecResult {
  CRASH,
  HANG,
  SUCCESS
}

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

fn exec(conf:&Conf) -> ExecResult {
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

  let one_sec = Duration::from_millis(conf.timeout); // TODO: Use Average timeout
  match child.wait_timeout(one_sec).expect("Wait Child") {
    Some(status) =>
      match status.unix_signal() {
        Some(_) => ExecResult::CRASH,
        None => ExecResult::SUCCESS
      },
    None => {
      child.kill().unwrap();
      child.wait().unwrap();
      ExecResult::HANG
    }
  }
}

fn check_interesting() -> bool {
  true
}

pub fn run_target(conf:&Conf, buf:&Vec<u8>) -> bool {
  setup_env(&conf, &buf);

  match exec(&conf) {
    ExecResult::CRASH => conf.save_crash(&buf),
    _ => ()
  };

  clear_env(&conf);
  check_interesting()
}
