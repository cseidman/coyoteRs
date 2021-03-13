use crate::opcodes::* ;
use crate::opcodes::OpCode::* ;
use crate::chunk::* ;
use crate::value::* ;
use crate::value::Value::* ;

pub fn disassembleChunk(chunk: &Chunk, name: &str) {

    println!("== {} ==", name) ;
    let mut i = 0 ;
    while i < chunk.code.len() {
        i = disassembleInstruction(chunk,i) ;
    }
    println!("== Done ==") ;
}

pub fn disassembleInstruction(chunk: &Chunk, offset: usize) -> usize {
    print!("{:04} ", offset);
    if offset > 0 && chunk.lines[offset] == chunk.lines[offset - 1] {
        print!("   | ");
    } else {
        print!("{:4} ", chunk.lines[offset]);
    }

    let raw_byte = chunk.code[offset];
    let instruction = OpCode::from_byte(raw_byte);
    let instruction_name = format!("{:?}", instruction);
    match instruction {
        OP_RETURN
        | OP_NEGATE
        | OP_IADD
        | OP_IMUL
        | OP_IDIV
        | OP_ISUB
        | OP_NIL
        | OP_TRUE
        | OP_FALSE
        | OP_NOT
        | OP_EQUAL
        | OP_GREATER
        | OP_LESS => return simpleInstruction(instruction_name.as_str(), offset),
        | OP_CONSTANT => constantInstruction(instruction_name.as_str(), chunk, offset),
        _ => {
            println!("Unknown code {:?}", raw_byte);
            return offset + 1;
        }
    }
}
fn simpleInstruction(name: &str, offset: usize) -> usize {
    println!("{}", name);
    return offset + 1;
}

fn constantInstruction(name: &str, chunk: &Chunk ,offset: usize) -> usize {
    let mut ar:[u8;2] = Default::default() ;
    ar.copy_from_slice(&chunk.code[offset+1..offset+3] ) ;
    let constant = u16::from_le_bytes(ar) as usize;
    print!("{:24} {:4} '", name, constant);
    let val = chunk.constants[constant] ;
    printValue(val);
    print!("'\n");
    return offset + 3 ;
}

pub fn printValue(val: Value) {

    match val {
        INTEGER(i) => print!("{}",i) ,
        DOUBLE(i) => print!("{}",i) ,
        NIL(_i) => print!("nil") ,
        BOOL(b) => if b {print!("True")} else {print!{"False"}}  ,
        _ => panic!("Unknown value type to print")
    }

}