extern crate byteorder;
extern crate memmap;

use std::fs;
use std::io::prelude::*;
use std::env;
use std::ffi::CString;
use std::os::raw::{c_char, c_int};

use std::io::Cursor;
use byteorder::{LittleEndian, ReadBytesExt};
use memmap::{Mmap, Protection};

use conf::Conf;

extern {
    fn initialize_libexec (argc: c_int, args: *const *const c_char, fd: c_int, timeout: u64);
    fn kill_forkserver();
    fn exec_fork(timeout: u64) -> c_int;
}

pub struct Feedback {
    pub exec_id    : u64,
    pub node        : u32,
    pub newnode : u32,
}

enum ExecResult {
    CRASH,
    HANG,
    SUCCESS
}

pub fn initialize(conf:&Conf) {
    let outputpath = format!("{}/.ret", conf.output_dir);
    {
        let mut f = fs::File::create(&outputpath).unwrap();
        let buf = [0 as u8; 16];
        f.write_all(&buf).unwrap();
    }
    env::set_var("BAEUM_RET_PATH", outputpath);

    let args = conf.args.iter()
                   .map(|ref s| CString::new(s.as_str()).unwrap()).collect::<Vec<CString>>();
    let argv = args.iter().map(|ref s| s.as_ptr()).collect::<Vec<*const c_char>>();
    unsafe {
        initialize_libexec(args.len() as c_int, argv.as_ptr() as *const *const c_char,
                           conf.stdin_fd as c_int, conf.timeout);
    }
}

pub fn finalize() {
    unsafe { kill_forkserver() };
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

// XXX : I want to use enum..
// Status = Timeout = -1 | Normal = 0 | SIGSEGV = 11 | SIGILL = 4 | SIGFPE = 8 | SIGABRT = 6
fn exec(conf:&Conf) -> ExecResult {
    let status = unsafe { exec_fork(conf.timeout) };
    match status {
        -1 => ExecResult::HANG,
        0 => ExecResult::SUCCESS,
        _ => ExecResult::CRASH,
    }
}

fn get_feedback(conf:&Conf) -> Feedback {
    // Better idea to get feedback? (always opening mmap ..?)
    let outputpath = format!("{}/.ret", conf.output_dir);
    let mmap = Mmap::open_path(outputpath, Protection::Read).unwrap();
    let bytes: &[u8] = unsafe { mmap.as_slice() };
    let mut buf = Cursor::new(&bytes[..]);
    let exec_id = buf.read_u64::<LittleEndian>().unwrap();
    let nodecount = buf.read_u32::<LittleEndian>().unwrap();
    let newnode = buf.read_u32::<LittleEndian>().unwrap();
    Feedback { exec_id: exec_id, node: nodecount, newnode: newnode }
}

pub fn run_target(conf:&Conf, buf:&Vec<u8>) -> Feedback {
    setup_env(&conf, &buf);

    let status = exec(&conf);
    let feedback = get_feedback(&conf);

    match status {
        ExecResult::CRASH => conf.save_crash(&buf, &feedback),
        _ => (),
    };

    clear_env(&conf);
    feedback
}
