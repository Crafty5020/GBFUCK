use std::process::exit;


#[derive(Debug, PartialEq, Eq)]
pub enum RunArgs {
	INVALID,
	Interpret,
	Compile
}
#[derive(Debug, PartialEq, Eq)]
pub enum Options {
	INVALID,
	Help,
	Version
}
impl Options {
	pub fn from_str(s: &str) -> Options {
		match s {
			"-h" | "--help" => Options::Help,
			"-v" | "--version" => Options::Version,
			_ => Options::INVALID,
		}
	}
}
impl RunArgs {
	pub fn from_str(s: &str) -> RunArgs {
		match s {
			"interpret"| "--interpret" | "-i" => RunArgs::Interpret,
			"compile" | "--compile" | "-c" => RunArgs::Compile,
			_ => RunArgs::INVALID,
		}
	}
}
pub fn help() {
	println!("Usage Interpreter:");
	println!("  gbfuck --interpret [main source file location] [options]");
	println!("  gbfuck -i [main source file location] [options]");
	println!("Usage Compiler:");
	println!("  gbfuck --compile [main source file location] [program name] [options]");
	println!("  gbfuck -c [main source file location] [program name] [options]");
	println!("Usage Options:");
	println!("  gbfuck [options]");
	
	println!("Options:");
	println!("  -h, --help     Show this help message");
	println!("  -v, --version  Show the version");
}
pub fn invalid_args() {
	eprintln!("Invalid arguments passed.");
	help();
	exit(1);
}
