// use std::io;

// pub fn adler32<R: std::io::Read>(mut reader: R) -> std::io::Result<u32> {
//     let mut hash = Adler32::new();
//     let mut buffer = [0u8; NMAX];
//     let mut read = reader.read(&mut buffer)?;
//     while read > 0 {
//         hash.write_bytes(&buffer[..read]);
//         read = reader.read(&mut buffer)?;
//     }
//     Ok(hash.sum32())
// }

// fn adler32_slow<R: io::Read>(reader: R) -> io::Result<u32> {
//     let mut a: u32 = 1;
//     let mut b: u32 = 0;

//     for byte in reader.bytes() {
//         let byte = byte? as u32;
//         a = (a + byte) % BASE;
//         b = (b + a) % BASE;
//     }

//     Ok((b << 16) | a)
// }

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
        let mut flag = true;
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
