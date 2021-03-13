use OpCode::* ;

#[derive(Debug)]
pub enum OpCode {
    OP_CONSTANT,
    OP_RETURN,
    OP_NEGATE,
    OP_IADD,
    OP_ISUB,
    OP_IMUL,
    OP_IDIV,
    OP_NIL,
    OP_TRUE,
    OP_FALSE,
    OP_GREATER,
    OP_EQUAL,
    OP_LESS,
    OP_NOT,
    OP_PUSH,
    OP_POP,
    OP_UNKNOWN

}

impl OpCode {

    pub fn to_byte(&self) -> u8 {
        match self {
            OP_CONSTANT     => 1,
            OP_RETURN       => 2,
            OP_NEGATE       => 3,
            OP_IADD         => 4,
            OP_ISUB         => 5,
            OP_IMUL        => 6,
            OP_IDIV         => 7,
            OP_NIL          => 8,
            OP_TRUE         => 9,
            OP_FALSE        => 10,
            OP_GREATER      => 11,
            OP_EQUAL        => 12,
            OP_LESS         => 13,
            OP_NOT          => 14,
            OP_PUSH         => 15,
            OP_POP          => 16,
            _ => 0
        }
    }

    pub fn from_byte(b:u8) -> OpCode {
        match b {
            0 => OP_UNKNOWN,
            1 => OP_CONSTANT,
            2 => OP_RETURN,
            3 => OP_NEGATE,
            4 => OP_IADD,
            5 => OP_ISUB,
            6 => OP_IMUL,
            7 => OP_IDIV,
            8 => OP_NIL,
            9 => OP_TRUE,
            10 => OP_FALSE,
            11 => OP_GREATER,
            12 => OP_EQUAL,
            13 => OP_LESS,
            14 => OP_NOT,
            15 => OP_PUSH,
            16 => OP_POP,
            _ => OP_UNKNOWN
        }
    }
}
