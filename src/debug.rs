
use crate::chunk::* ;
use crate::value::* ;

pub fn disassembleChunk(chunk: &Chunk, name: &str) {

    println!("== {} ==", name) ;
    let mut i = 0 ;
    while i < chunk.code.len() {
        i = disassembleIntruction(chunk,i) ;
    }
}

pub fn disassembleIntruction(chunk: &Chunk, offset: usize) -> usize {
    print!("{:04} ", offset) ;
    if offset > 0 && chunk.lines[offset] == chunk.lines[offset-1] {
        print!("   | ") ;
    } else {
        print!("{:4} ", chunk.lines[offset]);
    }

    let instruction = chunk.code[offset];
    match instruction {
        OP_RETURN => return simpleInstruction("OP_RETURN", offset),
        OP_NEGATE => return simpleInstruction("OP_NEGATE", offset),
        OP_ADD => return simpleInstruction("OP_ADD", offset),
        OP_MULTIPLY => return simpleInstruction("OP_MULTIPLY", offset),
        OP_DIVIDE => return simpleInstruction("OP_DIVIDE", offset),
        OP_SUBTRACT => return simpleInstruction("OP_SUBTRACT", offset),
        OP_CONSTANT => return constantInstruction("OP_CONSTANT", chunk, offset),
        _ => {
            println!("Unknown code {}", &instruction);
            return offset + 1 ;
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
    printValue(chunk.constants[constant]);
    print!("'\n");
    return offset + 3 ;
}

pub fn printValue(val: Value) {
    print!("{}",val) ;
}