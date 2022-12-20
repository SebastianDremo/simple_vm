use std::fs;

mod lex;
mod vm;

fn main() {
    let file_path = String::from("./../program.vm");
    let program = fs::read_to_string(file_path).expect("Cannot read from path {file_path}}"); 

    //println!("Program:");
    //println!("{}", program);

    let instructions = program.lines();

    let lexedInstructions = lex::lex_instructions(instructions);
} 

