use crate::value::ValueType;

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum BinopType {
    B_PLUS,
    B_MINUS,
    B_MULT,
    B_DIV,
    B_MOD
}

/**
valtypes expresses the value type we have in the left and right
side of the binary operation
*/
pub struct Binop {
    valtypes: (ValueType, ValueType),
    optype: BinopType,
}

impl Binop {
    pub fn new(v: BinopType, vtypes: (ValueType, ValueType)) -> Binop {
        return Binop {
            valtypes: vtypes,
            optype: v,
        };
    }
}