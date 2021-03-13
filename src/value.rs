use std::fmt;
use std::fmt::{Formatter, Error};
use std::convert::TryInto;

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum ValueType {
    VAL_BOOL,
    VAL_NIL,
    VAL_INTEGER,
    VAL_DOUBLE,
    VAL_STRING
}

/**
Value structure
valtype : ValueType
value : Vec<u8>

This is the object we put in the value type in the ast
It stores an enum value identifying the type the value is
and is keeps a vector of bytes to represent the structure

The "val_type" trait had functions

*/
pub struct Value {
    valtype: ValueType,
    value: Vec<u8>,
}

use ValueType::* ;

impl Value {

    /**
    Instantiate the struct with a value. As long as the value is on of the
    ones in ValueType, it'll take care of assigning the right enum member and
    converting the value to a vector of bytes
    */
    pub fn new<T: val_type>(v:T) -> Value {
        return Value{valtype: v.get_valtype(), value: v.to_bytes()} ;
    }
    // In case we need to check the type at compile-time
    pub fn get_type(&self) -> ValueType {
        return self.valtype.clone() ;
    }

    /**
    The following functions convert the struct to a concrete version of the
    data
    */
    pub fn to_integer(&self) -> i64 {
        return i64::from_le_bytes(  self.value
            .as_slice()
            .try_into()
            .unwrap()) ;
    }

    pub fn to_double(&self) -> f64 {
        return f64::from_le_bytes(  self.value
            .as_slice()
            .try_into()
            .unwrap()) ;
    }

    pub fn to_string(&self) -> String {
        return String::from_utf8_lossy(&self.value)
            .to_string() ;
    }

}

/**d
"val_type" trait to help treat the structs as values
*/
pub trait val_type {
    // Simply brings back the enum member indicating what kind of value it is
    fn get_valtype(&self) -> ValueType ;
    // Converts the base value to a vector of bytes
    fn to_bytes(&self) -> Vec<u8>;
}

impl val_type for i64 {
    fn get_valtype(&self) -> ValueType {
        return VAL_INTEGER ;
    }
    fn to_bytes(&self) -> Vec<u8>{
        return self.to_le_bytes().to_vec() ;
    }
}

impl val_type for f64 {
    fn get_valtype(&self) -> ValueType {
        return VAL_DOUBLE ;
    }
    fn to_bytes(&self) -> Vec<u8>{
        return self.to_le_bytes().to_vec() ;
    }
}

impl val_type for String {
    fn get_valtype(&self) -> ValueType {
        return VAL_STRING ;
    }

    fn to_bytes(&self) -> Vec<u8>{
        return self.clone().into_bytes() ;
    }
}

#[derive(PartialEq, Debug)]
pub enum Binop {
    PLUS, MINUS, DIV, MUL, MOD
}

impl fmt::Display for Binop {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            PLUS    => write!(f, "+"),
            MINUS   => write!(f, "-"),
            DIV     => write!(f, "/"),
            MUL     => write!(f, "*"),
            MOD     => write!(f, "%"),
        }
    }
}