use std::str::FromStr;

#[derive(Debug)]
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
    RET
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
            "RET" => Ok(Opcode::RET),
                _ => Err(())
        }
    }
}

pub struct Instruction {
    pub opcode: Opcode,
    pub val_1: i32, 
    pub val_2: i32
}


pub fn run_program(program: Vec<Instruction>) {
    let mut stack: Vec<i32> = Vec::new();
    for i in program {
        run_instruction(i, &mut stack); 
    }
}

fn run_instruction(i: Instruction, stack: &mut Vec<i32>) {
    match i.opcode {
        Opcode::PRINT => print(stack),
        Opcode::ADD => add(stack),  
        Opcode::PUSH => push(i.val_1, stack),
            _ => todo!()
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

fn push(value: i32, stack: &mut Vec<i32>) {
    stack.push(value); 
}

