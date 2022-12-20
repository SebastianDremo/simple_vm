use std::str::FromStr;

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


pub fn run_program(program: Vec<Instruction>, stack: Vec<i32>) {

}
