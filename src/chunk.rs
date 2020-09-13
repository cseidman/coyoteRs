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

impl Chunk {
    pub fn writeChunk(&mut self, byte: u8, line: usize) {
        
        // Check capacity - if we need more size
        if self.code.capacity() < self.code.len() + 1 {
            self.code.reserve(CODE_CAPACITY);
            self.lines.reserve(CODE_CAPACITY);
        }
        // Add the code to the end
        self.code.push(byte);
        self.lines.push(line);
        
    }

    pub fn writeConstant(&mut self, Index: u16, line: usize) {
        
        // Check capacity - if we need more size
        if self.code.capacity() < self.code.len() + 1 {
            self.code.reserve(CODE_CAPACITY);
            self.lines.reserve(CODE_CAPACITY);
        }
        // Add the code to the end
        self.code.append(&mut u16::to_le_bytes(Index).to_vec());
        self.code_ptr = self.code_ptr + 2;
        self.lines.push(line);
        self.lines.push(line);

    }

    pub fn addConstant(&mut self, value: Value) -> usize {

        // Check capacity - if we need more size
        if self.constants.capacity() < self.code.len() + 1 {
            self.constants.reserve(CONST_CAPACITY);
        }
        // Add the code to the end
        self.constants.push(value);
        self.const_ptr += 1;
        return self.code_ptr-1 ;
    }
}
