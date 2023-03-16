use crate::vm::{Instruction, Opcode};
use std::str::FromStr;
use core::str::Lines;


pub fn lex_instructions(instructions: Lines) -> Vec<Instruction> {
    let mut program: Vec<Instruction> = Vec::new();

    for instruction in instructions {
        println!("LOG: Lexing {}", instruction);
        let s: Vec<&str> = instruction.split(' ').collect();

        //jumps
        if s.contains(&"JMP") {
            println!("LOG: Lexing JMP instruction {} {}", s[0], s[1]);
            program.push(
                Instruction {
                    opcode: Opcode::from_str(s[0]).unwrap(),
                    val: -1,
                    jmp_val: s[1].to_owned()
                }
            );
        }
        //label
        else if instruction.contains(&":") {
            println!("LOG: Lexing LABEL instruction at {}", s[0]);
            program.push(
                Instruction {
                    opcode: Opcode::LABEL,
                    val: -1,
                    jmp_val: s[0].replace(":", "")
                }
            );           
        }
        //push
        else if s.len() == 2 {
            println!("LOG: Lexing PUSH instruction on {} {}", s[0], s[1]);
            program.push(
                Instruction {
                    opcode: Opcode::from_str(s[0]).expect("Create OPCODE enum from str"),
                    val: s[1].parse()
                        .expect(format!("ERROR: Should parse push with value {}", s[1]).as_str()),
                    jmp_val: String::new() //will not create any allocation 
                },
            );
        }
        //every other OP 
        else {
            println!("LOG: Lexing instruction at {}", s[0]);
            program.push(
                Instruction {
                    opcode: Opcode::from_str(s[0])
                        .expect("ERROR: Should lex other instructions without problems"),
                    val: -1,
                    jmp_val: String::new()
                    }
                );
        }

    }

    return program;
}
