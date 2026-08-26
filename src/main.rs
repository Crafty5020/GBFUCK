use std::env;
mod enums;
use enums::args;
mod interpreter;
fn main() {
    println!("Hello..?");
    println!("Are you there?");

    // Collect Args
    let cur_args: Vec<String> = env::args().collect();
    
    if cur_args.len() > 1 {
        println!("Arguments passed: {:?}", &cur_args[1..]);
        if 
    } else {
        println!("No arguments passed.");
        args::help();
    }
}
