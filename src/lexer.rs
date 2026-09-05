use std::{fs::File, io::Read, path::Path};
use crate::enums::opcode::BrainOpcodes;

pub fn lexerise(file_path: &Path) {
	let mut cur_file = File::open(file_path);

	match cur_file {
		Ok(cur_file) => {
			let mut opcodes: Vec<BrainOpcodes> = Vec::new();
			
			//for char in cur_file.bytes();
		}
		Err(error) => {
			println!("Error opening file: {}", error);
			println!("I trusted you with the truth. I TRUSTED YOU (4/35)");
			std::process::exit(1);
		}
	}

}

