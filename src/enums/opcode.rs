use std::{collections::HashMap, sync::OnceLock};

#[derive(Debug, Clone, PartialEq)]
pub enum BrainOpcodes {  // Opcodes used in the Brainfuck interpreter/compiler
	INVALID,
	IncrementPointer, // >
	DecrementPointer, // <
	IncrementValue, // +
	DecrementValue, // -
	OutputChar, // .
	InputChar, // ,
	LoopStart, // [
	LoopEnd, // ]
	Ignore, // ;
	Number(Vec<u32>), // @
	TreatAsNumber, // #
	Hex, // $
	Bytes(u8), // .db

}
#[derive(Debug, Clone, PartialEq)]
pub enum RenderOpcodes { // Opcodes used in the Brainfuck interpreter/compiler fr=or the renderer
	INVALID
}


static BRAIN_OPCODE_MAP: OnceLock<HashMap<String, BrainOpcodes>> = OnceLock::new();

impl BrainOpcodes {
    pub fn from_str(s: &str) -> BrainOpcodes {	
        match BRAIN_OPCODE_MAP.get() {
            Some(map) => match map.get(s) {
                Some(opcode) => opcode.clone(),
                None => BrainOpcodes::INVALID,
            },
            None => BrainOpcodes::INVALID,
        }
    }

    pub fn set_opcode_map() -> &'static HashMap<String, BrainOpcodes> {
        BRAIN_OPCODE_MAP.get_or_init(|| {
            HashMap::from([
                (">".to_string(), BrainOpcodes::IncrementPointer),
                ("<".to_string(), BrainOpcodes::DecrementPointer),
                ("+".to_string(), BrainOpcodes::IncrementValue),
                ("-".to_string(), BrainOpcodes::DecrementValue),
                (".".to_string(), BrainOpcodes::OutputChar),
                (",".to_string(), BrainOpcodes::InputChar),
                ("[".to_string(), BrainOpcodes::LoopStart),
                ("]".to_string(), BrainOpcodes::LoopEnd),
                (";".to_string(), BrainOpcodes::Ignore),
                ("@".to_string(), BrainOpcodes::Number(vec![])),
                ("#".to_string(), BrainOpcodes::TreatAsNumber),
                ("$".to_string(), BrainOpcodes::Hex),
                (".db".to_string(), BrainOpcodes::Bytes(u8::MAX)),
            ])
        })
    }

	pub fn predict_opcode(s: &str) -> Vec<BrainOpcodes> {
		// 1. Safely grab your map. If not initialized, return a single INVALID token.
		let Some(map) = BRAIN_OPCODE_MAP.get() else {
			return vec![BrainOpcodes::INVALID];
		};

		// 2. Create the peekable iterator over the characters
		let mut chars = s.chars().peekable();
		let mut tokens = Vec::new();

		// 3. Keep looping as long as there are characters left in the source string
		while let Some(_) = chars.peek() {
			let mut buffer = String::new();
			let mut matched_opcode: Option<BrainOpcodes> = None;

			// 4. Peeking loop: Accumulate characters step-by-step
			while let Some(&next_char) = chars.peek() {
				let mut test_buffer = buffer.clone();
				test_buffer.push(next_char);

				// Count how many keys in your map start with this prefix
				let prefix_matches = map.keys().filter(|key| key.starts_with(&test_buffer)).count();

				if prefix_matches == 0 {
					// If it leads to a dead end, stop checking further characters
					break;
				}

				// Valid prefix! Advance the iterator and permanently consume the character
				buffer.push(chars.next().unwrap());

				// Check if this current buffer matches an exact opcode perfectly
				if let Some(opcode) = map.get(&buffer) {
					matched_opcode = Some(opcode.clone());
					
					// Optimization: If this is the ONLY opcode that starts with this prefix, 
					// we don't need to peek further (e.g., if we hit '>' or 'PRINT\n')
					if prefix_matches == 1 {
						break;
					}
				}
			}

			// 5. Push the result to our token stream
			match matched_opcode {
				Some(opcode) => tokens.push(opcode),
				None => {
					// If the buffer has text but matched nothing, it's an error.
					// We consume the invalid character so we don't get stuck in an infinite loop.
					if buffer.is_empty() {
						chars.next(); 
					}
					tokens.push(BrainOpcodes::INVALID);
				}
			}
		}

		tokens
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