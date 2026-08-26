use std::{env, process::exit};
mod enums;
use enums::args;
mod interpreter;
mod options;
use options::{versions};

static VERSION: &str = "0.0.1PRE-DEV";

fn main() {
    println!("Hello..?");
    println!("Are you there? (3/35: Are you there?)");

    // Collect Args
    let cur_args: Vec<String> = env::args().collect();
    
    if cur_args[1..].len() > 1 {

        if args::RunArgs::from_str(&cur_args[1]) == args::RunArgs::Interpret || 
        args::RunArgs::from_str(&cur_args[1]) == args::RunArgs::Compile {
            match args::RunArgs::from_str(&cur_args[1]) {
                args::RunArgs::Interpret => {
                    println!("Interpreter selected.");
                    let mut inter = interpreter::InterpreterGV::new(, cur_args[3..].to_vec());
                },
                args::RunArgs::Compile => {
                    println!("Compiler selected.");
                    println!("Compiler is not yet implemented.");
                    println!("SO FUCK OFF");
                },
                _ => args::invalid_args()
            }
        } else if args::Options::from_str(&cur_args[1]) == args::Options::Help || 
        args::Options::from_str(&cur_args[1]) == args::Options::Version {
            match args::Options::from_str(&cur_args[1]) {
                args::Options::Help => args::help(),
                args::Options::Version => versions::show(),
                _ => args::invalid_args()
            }
        } else {
            args::invalid_args()
        }

    } else {
        println!("No arguments passed.");
        args::help();
    }
    println!("Hello, wait I mean bye");
    exit(0);
}
