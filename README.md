# Simple VM implemented in Rust

# PROJECT STILL IN DEVELOPMENT CHECK OPCODES SECTION TO SEE WHICH ONE ARE IMPLEMENTED SO FAR

The goal of this project is to learn some Rust and compare it to workflow
in C# (in which I code daily)

## Main goals
* As simple as possible
* Stack based
* Reads instrucions from file
* Instructions in form of opcodes

## OPCODES
- [x] ADD    | add two values at the top of stack
- [x] SUB    | subtract two values at the top of stack
- [x] MUL    | multiply two values at the top of stack
- [x] LT     | check if value at the top of the stack is less than second value on the stack
- [x] GT     | check if value at the top of the stack is less than second value on the stack
- [x] EQ     | check if value at the top of the stack is equal than second value on the stack
- [ ] JMP    | jump to given label
- [ ] JMPF   | jump to given label if value at the top of the stack is 0
- [ ] JMPT   | jump to given label if value at the top of the stack is 1
- [x] PRINT  | print value at the top of the stack
- [x] POP    | pop the value at the top of the stack
- [x] PUSH   | push the value to the top of the stack
- [x] STOP   | stop execution


## LABELS
VM implements labels to jump to with `JMP` `JMPF` and `JMPT`

## DEPENDENCIES
* Rustlang with cargo (at least cargo 1.66)
* Bash needed to execute `run.sh` script

## SETUP
Write instrucions in `program.vm` file.
Execute `./run` which will run the instrucions 

## EXAMPLE PROGRAM (NOT RUNNABLE YET)
```
PUSH 12
PUSH 20
ADD
PUSH 32
EQ
JMPT EQUAL 
JMPF NOT_EQUAL

EQUAL:
PUSH 1 
STOP

NOT_EQUAL:
PUSH 0 
STOP
```


