use crate::chunk::* ;
use crate::debug::* ;
use crate::value::* ;
use crate::compiler::* ;

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


    let mut chunk = newChunk();

    if !compile(source, &mut chunk) {
        return InterpretResult::INTERPRET_COMPILE_ERROR ;
    }

    let mut vm = VM {
        chunk,
        ip: 0,
        stack: [0;64000],
        stacktop: 0
    } ;

    let result = run(&mut vm) ;

    return result ;

}

fn push(vm: &mut VM, value: Value) {
    vm.stack[vm.stacktop] = value ;
    vm.stacktop+=1 ;
}

fn pop(vm: &mut VM) -> Value {
    vm.stacktop-=1 ;
    return vm.stack[vm.stacktop] ;
}

fn run(vm: &mut VM) -> InterpretResult {


    macro_rules! READ_BYTE {
    () => {{
                let code = vm.chunk.code[vm.ip];
                vm.ip+=1 ;
                code
          }}
    } ;

    macro_rules! READ_CONSTANT {
        () => {{
            let constant = vm.chunk.constants[u16::from_le_bytes([vm.chunk.code[vm.ip],vm.chunk.code[vm.ip+1]]) as usize] ;
            vm.ip+=2 ;
            constant
        }}
    }

    macro_rules! BINARY_OP {
        {$op:tt} => {
            let rval = pop(vm) ;
            let lval = pop(vm) ;
            push(vm,lval $op rval)
        }
    }

    loop {

        print!("          ");
        for slot in 0 .. vm.stacktop {
            print!("[ ");
            printValue(vm.stack[slot]);
            print!(" ]");
        }

        print!("\n");
        disassembleIntruction(&vm.chunk, vm.ip ) ;

        let instruction = READ_BYTE!() ;

        match instruction {
            OP_RETURN => {
                println!("{}",pop(vm)) ;
                return InterpretResult::INTERPRET_OK ;
            },
            OP_CONSTANT => {
                let constant = READ_CONSTANT!();
                push(vm,constant);
                print!("\n");
            },

            OP_ADD => {BINARY_OP!(+);},
            OP_SUBTRACT => {BINARY_OP!(-);},
            OP_MULTIPLY => {BINARY_OP!(*);},
            OP_DIVIDE => {BINARY_OP!(/);},

            OP_NEGATE => {
                let val = -pop(vm) ;
                push(vm,val);
            },
            _ => {
                return InterpretResult::INTERPRET_RUNTIME_ERROR ;
            }
        }
    }

}