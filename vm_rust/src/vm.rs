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

pub fn run_program(program: Vec<(Opcode, i32, i32)>, stack: Vec<i32>) {

}
