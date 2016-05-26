#![feature(expand_open_options)]
#![feature(libc)]
extern crate libc;

use libc::{mkfifo, open};
use libc::{O_NONBLOCK, O_RDONLY};
use std::ffi::CString;
use std::os::unix::fs::*;
use std::fs::*;

fn mk_fifo() -> i32 {
    return unsafe {
        let filename = CString::new("./fifo").unwrap().as_ptr();
        mkfifo(filename, 0o666);
        open(filename, O_NONBLOCK, O_RDONLY)
    };
}

fn open_options() -> File {
    return OpenOptions::new()
                    .create(true)
                    .custom_flags(O_NONBLOCK)
                    .open("./fifoo")
                    .unwrap();
}

fn main() {
    println!("{:?}", mk_fifo());
    println!("{:?}", open_options());
}
