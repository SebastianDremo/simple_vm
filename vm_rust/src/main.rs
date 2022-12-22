use std::fs;
use std::process;

mod lex;
mod vm;

fn main() {
    let exit_code = safe_main();
    process::exit(exit_code); 
} 

fn safe_main() -> i32 {
    let file_path = String::from("./../program.vm");
    let program = fs::read_to_string(file_path).expect("Cannot read from path {file_path}}"); 

    let lexed_instructions = lex::lex_instructions(program.lines());
    
    let exit_code = vm::run_program(lexed_instructions);
    return exit_code;
}

