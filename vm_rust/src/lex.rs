use std::str::Lines;
use crate::vm;
use std::str::FromStr;

pub fn lex_instructions(instructions: Lines) -> Vec<vm::Instruction> {
    let mut lexed_instructions = Vec::new();
    for instruction in instructions {
        if instruction.is_empty() {
            continue;
        }

        let s = instruction.split(' ').collect::<Vec<_>>(); 

        if s.len() == 3 {
            lexed_instructions.push(
                vm::Instruction {
                    opcode: vm::Opcode::from_str(s[0]).unwrap(),
                    val_1: s[1].parse().unwrap(),
                    val_2: s[2].parse().unwrap()}
                ); 
            }
        else if s.len() == 2 {
            lexed_instructions.push(
                vm::Instruction {
                    opcode: vm::Opcode::from_str(s[0]).unwrap(),
                    val_1: s[1].parse().unwrap(),
                    val_2: -1
                    }
                ); 
        }
        else if s.len() == 1 {
            lexed_instructions.push(
                vm::Instruction {
                    opcode: vm::Opcode::from_str(s[0]).unwrap(),
                    val_1: -1,
                    val_2: -1
                    }
                ); 
        }
    }

    return lexed_instructions;
}

