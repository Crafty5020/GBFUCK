use std::{env, process::exit};
mod enums;
use enums::args;
mod interpreter;
mod options;
use options::{versions};

use crate::enums::opcode;
mod lexer;
//use lexer::{FILE,lexerise};

static VERSION: &str = "0.0.1PRE-DEV";

fn main() {
    println!("Hello..?");
    println!("Are you still there? (3/35: Are you still there?)");
    // Collect Args
    let cur_args: Vec<String> = env::args().collect();
    opcode::BrainOpcodes::set_opcode_map();
    let predict_result_1 = opcode::BrainOpcodes::predict_opcode(".");
    let predict_result_2 = opcode::BrainOpcodes::predict_opcode(".d");
    let predict_result_3 = opcode::BrainOpcodes::predict_opcode(".db");
    println!("Predict result 1: {:?}", predict_result_1);
    println!("Predict result 2: {:?}", predict_result_2);
    println!("Predict result 3: {:?}", predict_result_3);
    if cur_args[1..].len() > 1 {

        if args::RunArgs::from_str(&cur_args[1]) == args::RunArgs::Interpret || 
        args::RunArgs::from_str(&cur_args[1]) == args::RunArgs::Compile {
            match args::RunArgs::from_str(&cur_args[1]) {
                args::RunArgs::Interpret => {
                    println!("Interpreter selected.");
                    //opcode::BrainOpcodes::set_opcode_map();
                    //let mut lexer = lexer::Lexer::new(cur_args[2].clone());
                    //let mut inter = interpreter::InterpreterGV::new(file, cur_args[3..].to_vec());
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
