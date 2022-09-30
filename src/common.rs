use crate::fio::FileIO;
use super::utils::Utility;
use super::constants::Val;

pub struct CmdArgs {
    pub chunk_size: usize,
    pub original_file_size: usize,
    pub modified_file_size: usize,
    pub original_file_path: String,
    pub modified_file_path: String,
}

impl CmdArgs {
    pub fn new() -> Self {
        Self {
            original_file_size: 0,
            modified_file_size: 0,
            chunk_size: Val::DEFAULT_C_SIZE,
            original_file_path: "".to_owned(),
            modified_file_path: "".to_owned(),
        }
    }
    
    pub fn parse(&mut self) -> Option<()>{
        let mut args = std::env::args();
        let program = args.nth(0).unwrap();
        let file1 = args.nth(0); 
        let file2 = args.nth(0);
        let opt_arg = args.nth(0);

        if file1.is_none() || file2.is_none() {
            Self::print_usage(program);
            return None;
        }

        self.original_file_path = file1.unwrap();    
        self.modified_file_path = file2.unwrap();   
        

        // here check files exist, and chunk size validity 
        let res = FileIO::get_file_size(&self.original_file_path);
        if res.is_none() {
            return None;
        }
        self.original_file_size = res.unwrap();
        
        let res = FileIO::get_file_size(&self.modified_file_path);
        
        if res.is_none() {
            return None;
        }
        self.modified_file_size = res.unwrap();

        if let Some(csz) = opt_arg {
            let parsed = csz.parse::<usize>();
            if parsed.is_ok() {
                self.chunk_size = parsed.unwrap();
            } else {
                println!("{:?}", parsed);
                Self::print_usage(program);
                return None;
            }
        }
        if !self.has_enough_chunks(self.original_file_size) {
            println!(
                "File: {}, must have at-least {} chunks!", 
                self.original_file_path, 
                Val::MIN_NUM_OF_CHUNKS
            );
            return None;
        }

        if !self.has_enough_chunks(self.modified_file_size) {
            println!(
                "File: {}, must have at-least {} chunks!", 
                self.modified_file_path, 
                Val::MIN_NUM_OF_CHUNKS
            );
            return None;
        }

        Some(())
    }

    fn print_usage(program: String) {
        println!("
        USAGE: {0} <file_1_path> <file_2_path> <optional chunk_size>
        
        Examples:
            {0} abc.txt def.txt
            {0} some.txt other.txt 4
            {0} some.bin other.bin 7
        
        ", program);
    }

    fn has_enough_chunks(&self, f_size: usize) -> bool {
        Utility::get_num_of_chunks(
            f_size, 
            self.chunk_size
        ) >= Val::MIN_NUM_OF_CHUNKS
    }
}


#[cfg(test)]
mod common_test {
    
}