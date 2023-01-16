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

//TODO jumps will have &str value not i32
pub struct Instruction {
    pub opcode: Opcode,
    pub val: i32,
    pub jmp_val: String
}

pub fn run_program(program: HashMap<String, Instruction>) {
    let mut stack: Vec<i32> = Vec::new();
    for (key, instr) in program {
        match instr.opcode {
            Opcode::JMP => key = instr.jmp_val,
            Opcode::JMPT => { if stack.last().unwrap().to_owned() == 1 { key = instr.jmp_val }},
            Opcode::JMPF => { if stack.last().unwrap().to_owned() == 0 { key = instr.jmp_val }},
            _ => run_instruction(instr, &mut stack)
        }
    }
}

fn run_instruction(i: Instruction, stack: &mut Vec<i32>) {
    match i.opcode {
        Opcode::PRINT => print(stack),
        Opcode::ADD => add(stack),  
        Opcode::SUB => sub(stack),
        Opcode::MUL => mul(stack),
        Opcode::EQ => eq(stack),
        Opcode::LT => lt(stack),
        Opcode::GT => gt(stack),
        Opcode::PUSH => push(i.val, stack),
        Opcode::POP => pop(stack),
        Opcode::LABEL => return,
            _ => return
    }

}

fn print(stack: &mut Vec<i32>) {
    println!("{}", stack.last().unwrap());
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
