#![allow(unused_assignments)]

use std::fs::File;
use crate::utils::Err;
use crate::utils::Utility;
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
        // get file size from file meta-data
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
        c_size: usize,
        f_size: usize
    ) -> Option<Vec<Vec<u8>>> {
        let bytes = Self::read_file_to_bytes(path);
        if bytes.is_none() { return None; }
        let bytes = bytes.unwrap();
        
        let mut i = 0usize;
        let mut s = 0usize;
        let mut e = 0usize;
        let len = bytes.len();
        
        let mut buf = Vec::<Vec<u8>>::with_capacity(
            Utility::get_num_of_chunks(f_size, c_size)
        );
        
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
    use super::*;
    use crate::utils::Utility;

    const chunk_size: usize = 4;
    const original_size: usize = 34;
    const added_chars_size: usize = 59;
    const removed_chars_size: usize = 31;
    const original: &str = "./test/files/original.dat";
    const invalid_file: &str = "./test/files/invalid.dat";
    const added_chars: &str = "./test/files/added_chars.dat";
    const removed_chars: &str = "./test/files/removed_chars.dat";

    #[test]
    fn ut_getFileSize_works() {
        let res0 = FileIO::get_file_size(original);
        let res1 = FileIO::get_file_size(added_chars);
        let res2 = FileIO::get_file_size(removed_chars);

        assert_ne!(res0, None);
        assert_ne!(res1, None);
        assert_ne!(res2, None);
        assert_eq!(res0.unwrap(), original_size);
        assert_eq!(res1.unwrap(), added_chars_size);
        assert_eq!(res2.unwrap(), removed_chars_size);
    }

    #[test]
    fn ut_fileToBytes_works() {
        let res = FileIO::read_file_to_bytes(original);
        assert_ne!(res, None);
        let res = res.unwrap();
        assert_eq!(res.len(), original_size);
        assert_eq!(
            res, 
            "sample data for rolling hash diff.".as_bytes().to_owned()
        );
    }

    #[test]
    fn ut_fileToChunkList_works() {
        let res = FileIO::read_file_to_chunk_list(
            original,
            chunk_size,
            original_size
        );
        assert_ne!(res, None);
        let res = res.unwrap();
        assert_eq!(
            res.len(), 
            Utility::get_num_of_chunks(original_size, chunk_size)
        );
        assert_eq!(
            res[0].len(),
            chunk_size
        );
        assert_eq!(
            res[0],
            "samp".as_bytes().to_owned()
        );
    }

    #[test]
    fn ut_getFileSizeWithInvalidFile_fails() {
        let res = FileIO::get_file_size(invalid_file);
        assert_eq!(res, None);
    }

    #[test]
    fn ut_fileToBytesWithInvalidFile_fails() {
        let res = FileIO::read_file_to_bytes(invalid_file);
        assert_eq!(res, None);
    }

    #[test]
    fn ut_fileToChunkListWithInvalidFile_fails() {
        let res = FileIO::read_file_to_chunk_list(
            invalid_file,
            0,
            0
        );
        assert_eq!(res, None);
    }
}
