use crate::compiler::* ;

pub type ParseFn = fn(&mut Compiler,bool) ;

#[derive(Copy, Clone)]
pub struct ParseRule {
    pub prefix:  Option<ParseFn>,
    pub infix:   Option<ParseFn>,
    pub prec:    Precedence,
}

#[derive(PartialEq, PartialOrd, Copy, Clone, Debug)]
pub enum Precedence {
    PREC_NONE=0,
    PREC_ASSIGNMENT,
    PREC_OR,
    PREC_AND,
    PREC_EQUALITY,
    PREC_COMPARISON,
    PREC_TERM,
    PREC_INCR,
    PREC_FACTOR,
    PREC_UNARY,
    PREC_CALL,
    PREC_ARRAY,
    PREC_INDEX,
    PREC_PRIMARY
}

use self::Precedence::* ;

pub static PREC_LIST: [Precedence;14] =
    [   PREC_NONE,
        PREC_ASSIGNMENT,
        PREC_OR,
        PREC_AND,
        PREC_EQUALITY,
        PREC_COMPARISON,
        PREC_TERM,
        PREC_INCR,
        PREC_FACTOR,
        PREC_UNARY,
        PREC_CALL,
        PREC_ARRAY,
        PREC_INDEX,
        PREC_PRIMARY
    ] ;