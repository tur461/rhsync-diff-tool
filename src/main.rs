#![allow(dead_code)]
#![allow(unused_imports)]

mod fio;
mod utils;
mod model;
mod traits;
mod common;
mod hashing;
mod constants;

use fio::FileIO;
use model::hash;
use model::chunk::Change;
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

    // sign list creation
    if sign.file_to_sign_list(
        &args.original_file_path,
        args.chunk_size
    ).is_none() { return; }

    // println!("sign list: {:#?}", sign.list);
    // delta list creation

    let mut diff_delta = DiffingDelta::new(&mut sign);

    if diff_delta.file_to_delta_list(
        &args.modified_file_path,
        args.chunk_size,
        args.original_file_size
    ).is_none() { return; }

    println!("delta list: {:#?}", diff_delta.list);
}

#[cfg(test)]
mod main_test {

}