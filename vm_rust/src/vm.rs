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

pub fn run_program(program: Vec<Instruction>) -> i32 {
    let mut stack: Vec<i32> = Vec::new();
    for i in program {
        let exit_code = run_instruction(i, &mut stack); 
        if exit_code != 0 {
            return exit_code;
        }
    }

    return 0;
}

fn run_instruction(i: Instruction, stack: &mut Vec<i32>) -> i32 {
    match i.opcode {
        Opcode::PRINT => print(stack),
        Opcode::ADD => add(stack),  
        Opcode::SUB => sub(stack),
        Opcode::MUL => mul(stack),
        Opcode::EQ => eq(stack),
        Opcode::LT => lt(stack),
        Opcode::GT => gt(stack),
        Opcode::PUSH => push(i.val_1, stack),
        Opcode::POP => pop(stack),
        Opcode::RET => return i.val_1,
            _ => todo!()
    }

    return 0; //all went fine
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
