use crate::value::* ;

const CODE_CAPACITY: usize = 1024000 ;
const CONST_CAPACITY: usize = 255 ;

pub const OP_CONSTANT: u8 = 0 ;
pub const OP_RETURN: u8 = 1 ;
pub const OP_NEGATE: u8 = 2 ;
pub const OP_ADD: u8 = 3 ;
pub const OP_SUBTRACT: u8 = 4;
pub const OP_MULTIPLY: u8 = 5;
pub const OP_DIVIDE: u8 = 6 ;

pub struct Chunk {
    pub code: Vec<u8>,
    pub code_ptr: usize,

    pub constants: Vec<Value>,
    pub const_ptr: usize,

    pub lines: Vec<usize>
}

pub fn newChunk() -> Chunk {
    return Chunk {
        code: Vec::<u8>::with_capacity(CODE_CAPACITY),
        constants: Vec::<Value>::with_capacity(CONST_CAPACITY),
        code_ptr: 0,
        const_ptr:0,
        lines: Vec::<usize>::with_capacity(CODE_CAPACITY)
    }
}

pub fn writeChunk(chunk: Chunk, byte: u8, line: usize) -> Chunk {

    let mut chnk = chunk ;

    // Check capacity - if we need more size
    if chnk.code.capacity() < chnk.code.len()+1 {
        chnk.code.reserve(CODE_CAPACITY) ;
        chnk.lines.reserve(CODE_CAPACITY) ;
    }
    // Add the code to the end
    chnk.code.push(byte);
    chnk.lines.push(line) ;
    return chnk ;

}

pub fn writeConstant(chunk: Chunk, Index: u16, line: usize) -> Chunk {

    let mut chnk = chunk ;

    // Check capacity - if we need more size
    if chnk.code.capacity() < chnk.code.len()+1 {
        chnk.code.reserve(CODE_CAPACITY) ;
        chnk.lines.reserve(CODE_CAPACITY) ;
    }
    // Add the code to the end
    chnk.code.append(&mut u16::to_le_bytes(Index).to_vec());
    chnk.code_ptr = chnk.code_ptr+2 ;
    chnk.lines.push(line) ;
    chnk.lines.push(line) ;

    return chnk ;

}

pub fn addConstant(chunk: Chunk, value: Value) -> Chunk {

    let mut chnk = chunk ;

    // Check capacity - if we need more size
    if chnk.constants.capacity() < chnk.code.len()+1 {
        chnk.constants.reserve(CONST_CAPACITY) ;
    }
    // Add the code to the end
    chnk.constants.push(value);
    chnk.const_ptr = chnk.const_ptr+1 ;

    return chnk ;

}
