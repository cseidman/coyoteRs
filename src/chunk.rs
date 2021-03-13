use crate::value::* ;

const CODE_CAPACITY: usize = 1024000 ;
const CONST_CAPACITY: usize = 255 ;

pub struct Chunk {
    pub code: Vec<u8>,
    pub code_ptr: usize,

    pub constants: Vec<Value>,
    const_ptr: usize,

    pub lines: Vec<usize>
}

pub fn newChunk() -> Chunk {
    return Chunk {
        code: Vec::<u8>::with_capacity(CODE_CAPACITY),
        code_ptr: 0,
        constants: vec![],
        lines: Vec::<usize>::with_capacity(CODE_CAPACITY),
        const_ptr: 0
    }
}

impl Chunk {

    fn check_capacity(&mut self) {
        // Check capacity - if we need more size
        if self.code.capacity() < self.code.len() + 1 {
            self.code.reserve(CODE_CAPACITY);
            self.lines.reserve(CODE_CAPACITY);
        }
    }

    pub fn writeChunk(&mut self, byte: u8, line: usize) {
        
        self.check_capacity() ;

        // Add the code to the end
        self.code.push(byte);
        self.lines.push(line);
        
    }

    pub fn writeConstant(&mut self, Index: u16, line: usize) {

        self.check_capacity() ;

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
        return self.const_ptr-1 ;
    }
}
