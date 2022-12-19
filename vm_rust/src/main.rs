use std::fs;
use std::str::Lines;

mod vm;

fn main() {
    let file_path = String::from("./../program.vm");
    let program = fs::read_to_string(file_path).expect("Cannot read from path {file_path}}"); 

    println!("Program:");
    println!("{}", program);

    let instructions = program.lines();

    lex_instructions(instructions)
} 


fn lex_instructions(instructions: Lines) {
    for instruction in instructions {
                 
    }
}

