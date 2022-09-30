#![allow(unused_assignments)]

use std::fs::File;
use crate::utils::Err;
use std::io::{Read, BufReader};
use crate::constants::{ErrKind};

pub struct FileIO;

impl FileIO {
    pub fn get_file_size(path: &str) -> Option<usize> {
        let f = File::open(path);
        if f.is_err() {
            Err::handle(path, ErrKind::FILE_OPEN);
            return None;
        }
        
        Some(f.unwrap().metadata().ok()?.len() as usize)
    }

    pub fn read_file_to_bytes(path: &str) -> Option<Vec<u8>> {
        let f = File::open(path);
        if f.is_err() {
            println!("{:#?}", f);
            Err::handle(path, ErrKind::FILE_OPEN);
            return None;
        }
        let mut reader = BufReader::new(f.unwrap());
        let mut buffer = Vec::new();
        reader.read_to_end(&mut buffer).ok()?;

        Some(buffer)
    }

    pub fn read_file_to_chunk_list(
        path: &str, 
        c_size: usize
    ) -> Option<Vec<Vec<u8>>> {
        let f_size = Self::get_file_size(path);
        if f_size.is_none() { return None;}
        let f_size = f_size.unwrap();

        let bytes = Self::read_file_to_bytes(path);
        if bytes.is_none() { return None; }
        let bytes = bytes.unwrap();
        let mut i = 0usize;
        let mut s = 0usize;
        let mut e = 0usize;
        let len = bytes.len();
        let mut buf = Vec::<Vec<u8>>::with_capacity((f_size as f32/c_size as f32).ceil() as usize);
        while i < len {
            s = i * c_size;
            e = (i * c_size) + c_size;
            if e >= len {
                e = len;
                i = len;
            }
            let mut v = Vec::<u8>::with_capacity(c_size);
            v.extend_from_slice(&bytes[s..e]);
            buf.push(v);
            i += 1;
        }
        Some(buf)
    }
}

#[cfg(test)]
mod fio_test {
    
}
