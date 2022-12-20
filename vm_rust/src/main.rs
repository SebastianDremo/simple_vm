use std::fs;

mod lex;
mod vm;

fn main() {
    let file_path = String::from("./../program.vm");
    let program = fs::read_to_string(file_path).expect("Cannot read from path {file_path}}"); 

    let lexed_instructions = lex::lex_instructions(program.lines());
    
    vm::run_program(lexed_instructions);
} 

