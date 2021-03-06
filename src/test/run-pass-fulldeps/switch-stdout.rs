// Copyright 2012-2014 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![feature(rustc_private)]

extern crate rustc_back;

use std::fs::File;
use std::io::{Read, Write};

use rustc_back::tempdir::TempDir;

#[cfg(unix)]
fn switch_stdout_to(file: File) {
    use std::os::unix::prelude::*;

    extern {
        fn dup2(old: i32, new: i32) -> i32;
    }

    unsafe {
        assert_eq!(dup2(file.as_raw_fd(), 1), 1);
    }
}

#[cfg(windows)]
fn switch_stdout_to(file: File) {
    use std::os::windows::prelude::*;

    extern "system" {
        fn SetStdHandle(nStdHandle: u32, handle: *mut u8) -> i32;
    }

    const STD_OUTPUT_HANDLE: u32 = (-11i32) as u32;

    unsafe {
        let rc = SetStdHandle(STD_OUTPUT_HANDLE,
                              file.into_raw_handle() as *mut _);
        assert!(rc != 0);
    }
}

fn main() {
    let td = TempDir::new("foo").unwrap();
    let path = td.path().join("bar");
    let f = File::create(&path).unwrap();

    println!("foo");
    std::io::stdout().flush().unwrap();
    switch_stdout_to(f);
    println!("bar");
    std::io::stdout().flush().unwrap();

    let mut contents = String::new();
    File::open(&path).unwrap().read_to_string(&mut contents).unwrap();
    assert_eq!(contents, "bar\n");
}
