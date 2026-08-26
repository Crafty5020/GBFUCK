pub enum BrainOpcodes {  // Opcodes used in the Brainfuck interpreter/compiler
	INVALID,
	IncrementPointer, // >
	DecrementPointer, // <
	IncrementValue, // +
	DecrementValue, // -
	OutputChar, // .
	InputChar, // ,,
	LoopStart, // [
	LoopEnd, // ]
	Ignore, // /
	Number(Vec<u32>), // @
	TreatAsNumber, // #
	Hex, // $
	Bytes(u8) // %
}
pub enum RenderOpcodes { // Opcodes used in the Brainfuck interpreter/compiler fr=or the renderer
	INVALID
}

impl BrainOpcodes {
	// Matches the next character into a brainfuck opcode, if it is not a valid opcode, it will return INVALID
	pub fn from_str(s: &str) -> BrainOpcodes {
		match s {
			">" => BrainOpcodes::IncrementPointer,
			"<" => BrainOpcodes::DecrementPointer,
			"+" => BrainOpcodes::IncrementValue,
			"-" => BrainOpcodes::DecrementValue,
			"." => BrainOpcodes::OutputChar,
			"," => BrainOpcodes::InputChar,
			"[" => BrainOpcodes::LoopStart,
			"]" => BrainOpcodes::LoopEnd,
			"/" => BrainOpcodes::Ignore,
			"@" => BrainOpcodes::Number(vec![]),
			"#" => BrainOpcodes::TreatAsNumber,
			"$" => BrainOpcodes::Hex,
			"%" => BrainOpcodes::Bytes(u8::MAX),
			_ => BrainOpcodes::INVALID
		}
	}
}

impl RenderOpcodes {
	// Matches the next character into a brainfuck renderer opcode, if it is not a valid opcode, it will return INVALID
	pub fn from_str(s: &str) -> RenderOpcodes {
		match s {
			_ => RenderOpcodes::INVALID,
		}
	}
}



// I would like to say this: I made a short story. Also MAN SHUT YOUR MOUTH UP IM TRYING TO CODE (2/35: Let me Code)