use std::str::Lines;
use crate::vm;
use std::str::FromStr;

pub fn lex_instructions(instructions: Lines) -> Vec<vm::Instruction> {
    let mut lexedInstructions = Vec::new();
    for instruction in instructions {
        let s = instruction.split(' ').collect::<Vec<_>>(); 

        if s.len() == 3 {
        lexedInstructions.push(
            vm::Instruction {
                opcode: vm::Opcode::from_str(s[0]).unwrap(),
                val_1: s[1].parse().unwrap(),
                val_2: s[2].parse().unwrap()}
            ); 
        }
        else if s.len() == 2 {
        lexedInstructions.push(
            vm::Instruction {
                opcode: vm::Opcode::from_str(s[0]).unwrap(),
                val_1: s[1].parse().unwrap(),
                val_2: -1
                }
            ); 
        }
    }

    return lexedInstructions;
}

