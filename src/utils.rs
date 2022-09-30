#![allow(dead_code)]
#![allow(unused_imports)]

use crate::constants::{ErrKind};

pub struct Utility;

impl Utility {
    pub fn panic(msg: &str) {
        panic!("{}", msg);
    }
}

pub struct Err;

impl Err {
    pub fn print_err(wh:&str, msg: std::io::Error) {
        println!("Error! {} {:?}",wh, msg);
    }

    pub fn handle(wh: &str, err_kind: ErrKind) {
        match err_kind {
            ErrKind::FILE_OPEN => Self::print_err(wh, std::io::Error::last_os_error()),
        }
    }
}
