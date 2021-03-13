use crate::chunk::* ;
use crate::debug::* ;
use crate::value::* ;
use crate::compiler::* ;
use crate::opcodes::* ;
use crate::opcodes::OpCode::* ;

use crate::value::ValueType::VAL_NIL;

pub struct VM {
    chunk: Chunk,
    ip: usize,

    pub stack: [Value;64000],
    stacktop: usize
}
#[derive(PartialEq)]
pub enum InterpretResult {
    INTERPRET_OK,
    INTERPRET_COMPILE_ERROR,
    INTERPRET_RUNTIME_ERROR
}

pub fn interpret(source: String) -> InterpretResult {

    let res = compile(source);
    if !res.is_ok() {
        return InterpretResult::INTERPRET_COMPILE_ERROR ;
    }

    let mut vm = VM {
        chunk: res.unwrap(),
        ip: 0,
        stack: [Value::OBJ(Obj{vtype: VAL_NIL, oref: 0});64000],
        stacktop: 0
    } ;

    let result = vm.run() ;

    return result ;

}

impl VM {
    fn push(&mut self, value: Value) {
        self.stack[self.stacktop] = value;
        self.stacktop += 1;
    }

    fn pop(&mut self) -> Value {
        self.stacktop -= 1;
        let top = self.stacktop ;
        let val = self.stack[top] ;
        return val;
    }

    fn peek(&mut self, distance: usize) -> Value {
        let dist = self.stacktop -1 - distance;
        return self.stack[dist];
    }

    fn run(&mut self) -> InterpretResult {
        macro_rules! READ_BYTE {
        () => {{
                let code = self.chunk.code[self.ip];
                self.ip+=1 ;
                code
        }}
        }

        macro_rules! READ_CONSTANT {
        () => {{
            let constant = self
                .chunk
                .constants[u16::from_le_bytes(
                    [self
                     .chunk
                     .code[self.ip],self.chunk.code[self.ip+1]
                    ]) as usize] ;
            self.ip+=2 ;
            constant
        }}
        }

        macro_rules! BINARY_OP {
        {$macroType:tt,$valType:tt,$op:tt} => {
            let rval = $macroType!(self.pop()) ;
            let lval = $macroType!(self.pop()) ;
            self.push($valType!((lval $op rval)))
        }
    }

        loop {
            print!("          ");
            for slot in 0..self.stacktop {
                print!("[ ");
                printValue(self.stack[slot]);
                print!(" ]");
            }

            print!("\n");
            disassembleInstruction(&self.chunk, self.ip);

            let instruction = OpCode::from_byte(READ_BYTE!());
            match instruction {
                OP_RETURN => {
                    println!("{}", AS_INTEGER!(self.pop()));
                    return InterpretResult::INTERPRET_OK;
                },
                OP_CONSTANT => {
                    let constant = READ_CONSTANT!();
                    self.push(constant);
                },

                OP_IADD => { BINARY_OP!(AS_INTEGER,INTEGER_VAL,+); },
                OP_ISUB => { BINARY_OP!(AS_INTEGER,INTEGER_VAL,-); },
                OP_IMUL => { BINARY_OP!(AS_INTEGER,INTEGER_VAL,*); },
                OP_IDIV => { BINARY_OP!(AS_INTEGER,INTEGER_VAL,/); },

                OP_NIL => { self.push(NIL_VAL!()); },
                OP_TRUE => { self.push(BOOL_VAL!(true)); },
                OP_FALSE => { self.push(BOOL_VAL!(false)); },

                OP_GREATER=> { BINARY_OP!(AS_BOOL,BOOL_VAL,>); },
                OP_LESS => { BINARY_OP!(AS_BOOL,BOOL_VAL,<); },
                OP_EQUAL=> {
                    let y = self.pop() ;
                    let x = self.pop() ;
                    self.push(BOOL_VAL!(x==y)) ;
                }

                OP_NEGATE => {
                    let val = -AS_INTEGER!(self.pop());
                    self.push(INTEGER_VAL!(val));
                },
                _ => {
                    return InterpretResult::INTERPRET_RUNTIME_ERROR;
                }
            }
        }
    }
}
