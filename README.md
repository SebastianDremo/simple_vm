# Simple VM implemented in Rust and C# 

The goal of this project is to learn some Rust and compare it to how efficient it is versus C# (in which I code daily)

## Main goals
* As simple as possible
* Stack based
* Reads instrucions from file
* Instructions can be in form of opcodes and bytecode

## OPCODES
* ADD    | add two values at the top of stack
* SUB    | subtract two values at the top of stack
* MUL    | multiply two values at the top of stack
* LT     | check if value at the top of the stack is less than second value on the stack
* GT     | check if value at the top of the stack is less than second value on the stack
* EQ     | check if value at the top of the stack is equal than second value on the stack
* JMP    | jump to given address
* JMPF   | jump to given address if value at the top of the stack is 0
* JMPT   | jump to given address if value at the top of the stack is 1
* PRINT  | print value at the top of the stack
* POP    | pop the value at the top of the stack
* PUSH   | push the value to the top of the stack
* RET    | stop execution and return with value

## SETUP
Write instrucions in `program.vm` file.
Execute `./run` which will run the instrucions in both implementations and compare the results

