use std::fs::File;

use crate::enums::args;
// INTERPRETER
pub struct InterpreterGV { // Global Variables for the interpreter
	// Args
	pub args: Vec<String>,
	// Memory Tape
	pub tape: Vec<u32>,
	// The current position of the pointer in the memory tape
	pub pointer: usize,
	// The current position of the pointer in the vmemory tape
	pub v_tape: Vec<u32>,
	// The current position of the pointer in the vmemory tape
	pub v_pointer: usize,
	// Current file being interpreted
	cur_file: File,

	// Hi welcome if you DISCORVERED the soure code CONGRATS NOT bc this is the first fucking files there are more easter eggs though (1/35: Hi)
}

impl InterpreterGV {

	// new fuction nothing special
	pub fn new(file: File,args: Vec<String>) -> InterpreterGV {
		InterpreterGV {

			tape: vec![0; 8_000_000],
			pointer: 0,
			v_tape: vec![0; 4_000_000],
			v_pointer: 0,
			cur_file: file,
		}
	}

	// adds 1 to the current block in the memory tape or vmemory tape depending on the vram boolean (If true then its the vram if false then regular ram)
	pub fn add_to_block(&mut self, vram: bool) {
		if vram {
			self.v_tape[self.v_pointer] += 1;
		} else {
			self.tape[self.pointer] += 1;
		}
	}

	// subtracts 1 from the current block in the memory tape or vmemory tape depending on the vram boolean (If true then its the vram if false then regular ram)
	pub fn subtract_from_block(&mut self, vram: bool) {
		if vram {
			self.v_tape[self.v_pointer] -= 1;
		} else {
			self.tape[self.pointer] -= 1;
		}
	}
	// moves the pointer to the right in the memory tape or vmemory tape depending on the vram boolean (If true then its the vram if false then regular ram)
	pub fn move_pointer_right(&mut self, vram: bool) {
		if vram {
			self.v_pointer += 1;
		} else {
			self.pointer += 1;
		}
	}
	// moves the pointer to the left in the memory tape or vmemory tape depending on the vram boolean (If true then its the vram if false then regular ram)
	pub fn move_pointer_left(&mut self, vram: bool) {
		if vram {
			self.v_pointer -= 1;
		} else {
			self.pointer -= 1;
		}
	}

}