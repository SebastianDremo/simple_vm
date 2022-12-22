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

        if s.len() == 2 {
            lexed_instructions.push(
                vm::Instruction {
                    opcode: vm::Opcode::from_str(s[0])
                        .expect(&format!("Could not lex {}", s[0])),
                    val: s[1].parse().unwrap()
                    }
                ); 
        }
        else if s.len() == 1 && s[0].contains(":") { // for labels
            lexed_instructions.push(
                vm::Instruction {
                    opcode: vm::Opcode::LABEL,
                    val: lexed_instructions.len(), //index of label in instructions vec
                    s_val: String::from(s[0])
                    }
        }
        else if s.len() == 1 {
            lexed_instructions.push(
                vm::Instruction {
                    opcode: vm::Opcode::from_str(s[0])
                        .expect(&format!("Could not lex {}", s[0])),
                    val: -1
                    }
                ); 
        }
    }

    return lexed_instructions;
}

