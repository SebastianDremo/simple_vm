use std::str::Lines;
use crate::vm;
use std::str::FromStr;

pub fn lex_instructions(instructions: Lines) -> Vec<vm::Instruction> {
    let mut lexed_instructions: HashMap<String, vm::Instruction> = HashMap::new();
    for (i, instruction) in instructions.iter().enumerate() {
        if instruction.is_empty() {
            continue;
        }

        let s = instruction.split(' ').collect::<Vec<_>>(); 

        //push
        if s.len() == 2 {
            lexed_instructions.insert(
                i.to_string(),
                vm::Instruction {
                    opcode: vm::Opcode::from_str(s[0])
                        .expect(&format!("Could not lex push op {}", s[0])),
                    val: s[1].parse().unwrap()
                    }
                ); 
        }
        //labels
        else if s.contains(":") {
            lexed_instructions.insert(
                s[0].replace(":", ""), //no need for ':' in key
                vm::Instruction {
                    Opcode::LABEL,
                    val: -1
                    }
                ); 
        }
        //everything else
        else if s.len() == 1 {
            lexed_instructions.insert(
                i.to_string(),
                vm::Instruction {
                    opcode: vm::Opcode::from_str(s[0])
                        .expect(&format!("Could not lex op {}", s[0])),
                    val: -1
                    }
                ); 
        }
    }

    return lexed_instructions;
}

