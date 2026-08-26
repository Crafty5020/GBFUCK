use std::process::exit;

use crate::VERSION;

pub fn show() {
	println!("GBFUCK Version: {}", VERSION);
	exit(0);
}