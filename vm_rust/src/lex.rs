use crate::vm::{Instruction, Opcode};
use std::{
    collections::HashMap,
    str::{FromStr, Lines},
};

pub fn lex_instructions(instructions: Lines) -> HashMap<String, Instruction> {
    let mut program: HashMap<String, Instruction> = HashMap::new();

    for (i, instruction) in instructions.enumerate() {
        let s: Vec<&str> = instruction.split(' ').collect();

        //push
        if s.len() == 2 {
            program.insert(
                i.to_string(),
                Instruction {
                    opcode: Opcode::from_str(s[0]).unwrap(),
                    val: s[1].parse().unwrap()
                },
            );
        }
        //label
        else if s.contains(&":") {
            program.insert(
                s[0].replace(":", ""),
                Instruction {
                    opcode: Opcode::LABEL,
                    val: -1
                }
            );           
        }
        //every other OP 
        else {
            program.insert(
                i.to_string(),
                Instruction {
                    opcode: Opcode::from_str(s[0]).unwrap(),
                    val: -1 
                    }
                );     
        }

    }

    return program;
}
