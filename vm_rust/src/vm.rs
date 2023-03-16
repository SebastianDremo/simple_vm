use std::collections::HashMap;
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq)]
pub enum Opcode {
    ADD,
    SUB, 
    MUL, 
    LT, 
    GT, 
    EQ, 
    JMP, 
    JMPF, 
    JMPT, 
    PRINT, 
    POP, 
    PUSH, 
    STOP,
    LABEL
}

impl FromStr for Opcode {
    type Err  = ();

    fn from_str(input: &str) -> Result<Opcode, Self::Err> {
        match input {
            "ADD" => Ok(Opcode::ADD), 
            "SUB" => Ok(Opcode::SUB),
            "MUL" => Ok(Opcode::MUL), 
            "LT" => Ok(Opcode::LT),
            "GT" => Ok(Opcode::GT), 
            "EQ" => Ok(Opcode::EQ),
            "JMP" => Ok(Opcode::JMP), 
            "JMPF" => Ok(Opcode::JMPF),
            "JMPT" => Ok(Opcode::JMPT), 
            "PRINT" => Ok(Opcode::PRINT),
            "POP" => Ok(Opcode::POP), 
            "PUSH" => Ok(Opcode::PUSH),
            "STOP" => Ok(Opcode::STOP),
            "LABEL" => Ok(Opcode::LABEL),
                _ => Err(())
        }
    }
}

pub struct Instruction {
    pub opcode: Opcode,
    pub val: i32,
    pub jmp_val: String
}

pub fn run_program(program: Vec<Instruction>) {
    let mut stack: Vec<i32> = Vec::new();
    let mut tag_indexes: HashMap<&String, usize> = HashMap::new();
    let mut i: usize = 0;
    let program_size: usize = program.len();
    
    loop {
        match program[i].opcode {
            Opcode::PRINT => print(&mut stack),
            Opcode::ADD => add(&mut stack),  
            Opcode::SUB => sub(&mut stack),
            Opcode::MUL => mul(&mut stack),
            Opcode::EQ => eq(&mut stack),
            Opcode::LT => lt(&mut stack),
            Opcode::GT => gt(&mut stack),
            Opcode::PUSH => push(program[i].val, &mut stack),
            Opcode::POP => pop(&mut stack),
            Opcode::LABEL => {
                println!("LOG: Saving label {} in memory...", program[i].jmp_val);
                tag_indexes.insert(&program[i].jmp_val, i);
            }
            Opcode::JMP => {
                println!("LOG: Jumping to {}", program[i].jmp_val);
                i = tag_indexes.get(&program[i].jmp_val)
                    .expect("ERROR: Should find jmp label.").clone();
                continue;
            }
                _ => todo!()
        }

        i = i + 1;
        if i > program_size {
            break;
        }


    }
}


fn print(stack: &mut Vec<i32>) {
    println!("VM: {}", stack.last().unwrap());
}

fn add(stack: &mut Vec<i32>) {
    let a = stack.pop().unwrap();
    let b = stack.pop().unwrap();

    stack.push(a+b);
}

fn sub(stack: &mut Vec<i32>) {
    let a = stack.pop().unwrap();
    let b = stack.pop().unwrap();

    stack.push(b-a);
}

fn push(value: i32, stack: &mut Vec<i32>) {
    stack.push(value); 
}

fn pop(stack: &mut Vec<i32>) {
    stack.pop();
}

fn mul(stack: &mut Vec<i32>) {
    let a = stack.pop().unwrap();
    let b = stack.pop().unwrap();

    stack.push(a*b);
}

fn eq(stack: &mut Vec<i32>) {
    let a = stack.pop().unwrap();
    let b = stack.pop().unwrap();

    stack.push((a == b) as i32);
}

fn lt(stack: &mut Vec<i32>) {
    let a = stack.pop().unwrap();
    let b = stack.pop().unwrap();

    stack.push((a < b) as i32);
}

fn gt(stack: &mut Vec<i32>) {
    let a = stack.pop().unwrap();
    let b = stack.pop().unwrap();

    stack.push((a > b) as i32);
}
