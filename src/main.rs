#![allow(dead_code)]
#![allow(unused_imports)]

mod fio;
mod utils;
mod model;
mod traits;
mod common;
mod hashing;
mod constants;

use model::delta::DiffingDelta;
use model::signature::Signature;

use crate::constants::{Val};
use crate::common::{CmdArgs, };

fn main() {
    let mut args = CmdArgs::new();
    if args.parse().is_none() {
        return;
    }

    let mut sign = Signature::new();

    // ------------- sign list creation --------------
    if sign.file_to_sign_list(
        &args.original_file_path,
        args.chunk_size,
        args.original_file_size
    ).is_none() { return; }

    // ------------- delta list creation -------------
    let mut diff_delta = DiffingDelta::new(&mut sign);

    if diff_delta.file_to_delta_list(
        &args.modified_file_path,
        args.chunk_size
    ).is_none() { return; }

    println!("delta list: {:#?}", diff_delta.list);
}

#[cfg(test)]
mod main_test {
    use super::*;
    use crate::fio::FileIO;
    use crate::utils::Utility;
    use crate::model::changes::Change;
    use crate::hashing::adler::Adler32;
    use crate::hashing::x2hash::X2Hash64;
    
    const chunk_size: usize = 4;
    const original_size: usize = 34;
    const original: &str = "./test/files/original.dat";
    const added_chars: &str = "./test/files/added_chars.dat";
    const removed_chars: &str = "./test/files/removed_chars.dat";

    #[test]
    fn ut_createSignList_works() {
        let mut sign = Signature::new();
        let res = sign.file_to_sign_list(
            original,
            chunk_size,
            original_size
        );

        assert_ne!(res, None);
        assert_eq!(
            sign.list.len(), 
            Utility::get_num_of_chunks(
                original_size, 
                chunk_size
            )
        );
        
        let mut adler = Adler32::new();
        
        let first_chunk = &FileIO::read_file_to_bytes(
            &original
        ).unwrap()[0..chunk_size];
        //  check L1 hash
        assert_eq!(
            sign.list.get(0).unwrap().L1, 
            adler.write_bytes(first_chunk).sum32()
        );
        // check L2 hash
        assert_eq!(
            sign.list.get(0).unwrap().L2, 
            X2Hash64::sum64(first_chunk)
        );
    }

    #[test]
    fn ut_createDeltaListWithAdditions_works() {
        let mut sign = Signature::new();
        let res = sign.file_to_sign_list(
            &original,
            chunk_size,
            original_size
        );

        assert_ne!(res, None);

        let mut delta = DiffingDelta::new(&mut sign);

        let res = delta.file_to_delta_list(
            added_chars,
            chunk_size
        );

        assert_ne!(res, None);

        assert_eq!(delta.list.len(), 4);
        
        let ch0 = delta.get(0);
        assert_ne!(ch0, None);
        let ch0 = ch0.unwrap();
        assert_eq!(ch0.del_chunk, false);
        assert_eq!(ch0.before, Some(true));
        assert_eq!(ch0.add_or_del_idx, 16);
        assert_eq!(ch0.content, Some("foradded ".as_bytes().to_owned()));
    }

    #[test]
    fn ut_createDeltaListWithRemovals_works() {
        let mut sign = Signature::new();
        let res = sign.file_to_sign_list(
            &original,
            chunk_size,
            original_size
        );

        assert_ne!(res, None);

        let mut delta = DiffingDelta::new(&mut sign);

        let res = delta.file_to_delta_list(
            removed_chars,
            chunk_size
        );

        assert_ne!(res, None);

        assert_eq!(delta.list.len(), 4);
        
        let mut chs = Vec::<Change>::new();
        for ch in delta.list.iter() {
            chs.push(ch.clone());
        }
        // delete chunk at 20 in original
        assert_eq!(chs[3].del_chunk, true);
        assert_eq!(chs[3].before, None);
        assert_eq!(chs[3].add_or_del_idx, 20);
        assert_eq!(chs[3].content, None);
        // delete chunk at 0 in original
        assert_eq!(chs[2].del_chunk, true);
        assert_eq!(chs[2].before, None);
        assert_eq!(chs[2].add_or_del_idx, 0);
        assert_eq!(chs[2].content, None);
        // add content before chunk at 24 in original
        assert_eq!(chs[1].del_chunk, false);
        assert_eq!(chs[1].before, Some(true));
        assert_eq!(chs[1].add_or_del_idx, 24);
        assert_eq!(chs[1].content, Some("i ".as_bytes().to_owned()));
        // add content before chunk at 4 in original
        assert_eq!(chs[0].del_chunk, false);
        assert_eq!(chs[0].before, Some(true));
        assert_eq!(chs[0].add_or_del_idx, 4);
        assert_eq!(chs[0].content, Some("sap".as_bytes().to_owned()));
    }
}