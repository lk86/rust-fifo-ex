extern crate mio;
extern crate libc;
extern crate bytes;
extern crate nix;

use std::os::unix::io::FromRawFd;
use mio::tcp::TcpStream;
use mio::unix::PipeReader;
use mio::*;
use libc::{mkfifo, open};
use libc::{O_NONBLOCK, O_RDONLY};
use std::ffi::CString;
use std::fs::*;

const STDIN: Token = Token(0);

fn mk_fifo() -> i32 {
    return unsafe {
        let filename = CString::new("./fifo").unwrap().as_ptr();
        mkfifo(filename, 0o666);
        open(filename, O_NONBLOCK, O_RDONLY)
    };
}

fn main() {
    let mut event_loop = EventLoop::new().unwrap();
    let std_pipe = unsafe { PipeReader::from_raw_fd(mk_fifo()) };
    event_loop.register(&std_pipe, STDIN,
        EventSet::all() ^ EventSet::writable(), PollOpt::empty()).unwrap();
    //event_loop.run().unwrap();
}
