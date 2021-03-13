use crate::scanner::* ;
use crate::scanner::TokenType::*;
use crate::chunk::* ;
use crate::value::* ;
use crate::rules::* ;
use crate::rules::Precedence::* ;
use crate::debug::* ;
use crate::opcodes::* ;
use crate::opcodes::OpCode::* ;

use std::io::{self, Write};
use crate::ast::ast_node::BinaryOp;
use crate::value::ValueType::VAL_INTEGER;
use crate::ast::ast_tree;


#[derive(Clone)]
struct Parser {
    current: Token,
    previous: Token,
    hadError: bool,
    panicMode: bool
}

pub struct Compiler {
    scanner: Scanner,
    parser: Parser,
    chunk: Chunk,
    ast: ast_tree
}

impl Compiler {

    fn t_check(&self, t:TokenType) -> bool {
        return self.parser.current.toktype == t ;
    }

    fn t_match(&mut self, t:TokenType) -> bool {
         if self.t_check(t) {
             self.advance() ;
             return true ;
         } else {
             return false ;
         }
    }

    fn advance(&mut self) {

       self.parser.previous = self.parser.current.clone();
       self.parser.current = self.scanner.scanToken();
       if self.parser.current.toktype == T_ERROR {
            let msg = self.parser.current.name.clone();
            self.errorAtCurrent(msg.as_str());
       }

    }

    fn consume(&mut self, tokType: TokenType, message: &str) {
        if self.parser.current.toktype == tokType {
            self.advance();
            return;
        }

        self.errorAtCurrent(message);
    }

    fn endCompiler(&mut self) {
        if !self.parser.hadError {
            disassembleChunk(&self.chunk,"code");
        }
        self.emitReturn() ;
    }

    //fn double(&mut self) {
    //    let value:f64 = self.parser.previous.name.parse::<f64>().unwrap() ;
    //    self.emitConstant(value) ;
    //}

    /* Emit byte combos */
    fn emitByte(&mut self, op:OpCode) {
        self.chunk.writeChunk(op.to_byte(), self.parser.previous.line);
    }

    fn emitBytes(&mut self, op: OpCode, operand: u16) {
        let bytes = u16::to_le_bytes(operand) ;
        self.chunk.writeChunk(op.to_byte(), self.parser.previous.line);
        self.chunk.writeChunk(bytes[0],self.parser.previous.line);
        self.chunk.writeChunk(bytes[1],self.parser.previous.line);
    }

    fn emitReturn(&mut self) {
        self.emitByte(OP_RETURN) ;
    }

    fn emitConstant(&mut self, value: Value) {
        let operand = self.makeConstant(value) ;
        self.emitBytes(OP_CONSTANT, operand) ;
    }

    fn makeConstant(&mut self, value: Value) -> u16 {
        let constant = self.chunk.addConstant(value) as u16;
        if constant > u16::MAX {
            self.error("Too many constants in one chunk.");
            return 0;
        }
        return constant;
    }

    /* Expressions and statements */
    fn expression(&mut self) {
        self.ParsePrecedence(PREC_ASSIGNMENT) ;
    }

    fn import_module(&self) {
        unimplemented!()
    }

    fn declare_module(&self) {
        unimplemented!()
    }

    fn if_statement(&self) {
        unimplemented!()
    }

    fn declare_variable(&self) {
        unimplemented!()
    }

    fn expression_statement(&mut self) {
        self.expression();
    }

    fn statement(&mut self) {

        if self.t_match(T_MODULE) { self.declare_module()}
        else if self.t_match(T_IMPORT) { self.import_module()}
        else if self.t_match(T_LET) {self.declare_variable()}
        else if self.t_match(T_IF) {self.if_statement()}
        else if self.t_match(T_ELSE) {}
        else if self.t_match(T_FN) {}
        else if self.t_match(T_RETURN) {}
        else if self.t_match(T_LOOP) {}
        else {self.expression_statement()}

    }

    fn evaluate(&mut self) {
        self.statement() ;
    }

    pub fn unary(&mut self, _canAssign:bool) {
        let operatorType = self.parser.previous.toktype ;

        self.ParsePrecedence(PREC_UNARY);

        match operatorType {
            T_MINUS=> self.emitByte(OP_NEGATE),
            _ => panic!("Unknown operator type")
        }
    }

    pub fn binary(&mut self, _canAssign:bool) {

        let operatorType = self.parser.previous.toktype ;
        let rule = self.GetRule(operatorType) ;
        let rPrec = rule.prec as usize +1 ;
        self.ParsePrecedence(PREC_LIST[rPrec]) ;

        match operatorType {
            T_PLUS          =>  {
                // Check to see what type the expression should be by walking up the tree nodes

            }
            _ =>  println!("No match on binary OP??") // This really shouldn't happen
        }

        // Emit the operator instruction.
        match operatorType {

            T_PLUS          =>  self.emitByte(OP_IADD),
            T_MINUS         =>  self.emitByte(OP_ISUB),
            T_STAR          =>  self.emitByte(OP_IMUL),
            T_SLASH         =>  self.emitByte(OP_IDIV),

            T_BANG_EQUAL    => {self.emitByte(OP_EQUAL); self.emitByte(OP_NOT);},
            T_EQUAL_EQUAL   => self.emitByte(OP_EQUAL),
            T_GREATER       => self.emitByte(OP_GREATER),
            T_GREATER_EQUAL => {self.emitByte(OP_LESS); self.emitByte(OP_NOT);},
            T_LESS          => self.emitByte(OP_LESS),
            T_LESS_EQUAL    => {self.emitByte(OP_GREATER); self.emitByte(OP_NOT);},

            _ =>  println!("No match on binary OP??") // This really shouldn't happen
        }
    }

    pub fn grouping(&mut self, _canAssign:bool) {
        self.expression() ;
        self.consume(TokenType::T_RIGHT_PAREN, "Expect ')' after expression")
    }

    pub fn integer(&mut self, _canAssign:bool) {
        let value = Value::INTEGER(self.parser.previous.name.parse::<i64>().unwrap()) ;
        //ast_tree.push(value) ;
        //self.emitConstant(INTEGER_VAL!(value)) ;
    }

    pub fn double(&mut self, _canAssign:bool) {
        let value:f64 = self.parser.previous.name.parse::<f64>().unwrap() ;
        self.emitConstant(DOUBLE_VAL!(value)) ;
    }

    pub fn literal(&mut self, _canAssign:bool) {

        match self.parser.previous.toktype {
            T_FALSE=> self.emitByte(OP_FALSE),
            T_TRUE => self.emitByte(OP_TRUE),
            T_NIL => self.emitByte(OP_NIL),
            _ => self.error("Unknown literal")
        }
    }

    /* Makes the negation possible by checking that we're only negating logical values */
    fn isFalse(&mut self, value: Value) -> bool {
        return IS_NIL!(value) || (IS_BOOL!(value) && !AS_BOOL!(value))
    }

    fn GetRule(&self, t:TokenType) -> ParseRule {
        //println!("{}", t as usize);
        return t.get_rule() ;
    }

    fn ParsePrecedence(&mut self, prec:Precedence) {

        self.advance() ;

        // This loads the prefix rule which either contains a value such as
        // a variable or literal or a prefix that affects the next value
        let rule = self.GetRule(self.parser.previous.toktype);
        let prefix = rule.prefix;

        // This is an error in that an expression needs to at least begin
        // with a prefix rule
        if prefix.is_none() {
            self.error("Expect expression (no prefix)");
            return ;
        }
        let canAssign = prec <= PREC_ASSIGNMENT;
        prefix.unwrap()(self,canAssign);

        while prec <= (self.GetRule(self.parser.current.toktype)).prec {
            self.advance() ;
            println!("PREC (after advance): {:?}",self.parser.previous.toktype) ;
            let infix = self.GetRule(self.parser.previous.toktype).infix ;
            if infix.is_some()  {
                infix.unwrap()(self,canAssign) ;
            }

            if canAssign && self.t_match(T_EQUAL) {
               self.error("Invalid assignment target.") ;
            }
        }
    }

    /* Error management*/

    fn error(&mut self, message: &str) {
        self.errorAt(message) ;
    }

    fn errorAtCurrent(&mut self, message: &str) {
        self.errorAt(message);
    }

    fn errorAt(&mut self,message: &str) {

        if self.parser.panicMode {return;}
        self.parser.panicMode = true;

        use TokenType::* ;
        let _ = io::stderr().write_all(format!("[line {}] Error", &self.parser.current.line).as_bytes());

        if self.parser.current.toktype == T_EOF {
            let _ = io::stderr().write_all(b" at end");
        } else if self.parser.current.toktype == T_ERROR {
            // Nothing.
        } else {
            let _ = io::stderr().write_all(format!(" at '{}'", &self.parser.current.name).as_bytes());
        }

        let _ = io::stderr().write_all( format!(": {}\n", message).as_bytes());
        self.parser.hadError = true;
    }

}

pub fn compile(source: String) -> Result<Chunk,bool> {

    let scanner = newScanner(source);

    let mut compiler = Compiler {
        scanner,
        parser: Parser {
            current: Token{
                name: "Start".to_string(),
                toktype: TokenType::T_START,
                line: 0
            },
            previous: Token{
                name: "Start".to_string(),
                toktype: TokenType::T_START,
                line: 0
            },
            hadError: false,
            panicMode: false
        },
        chunk: newChunk(),
        ast: ast_tree::new()
    } ;

    use TokenType::* ;
    compiler.advance() ;
    loop {
        if compiler.t_match(T_EOF) {
            compiler.endCompiler() ;
            break ;
        }
        compiler.evaluate();
    }

    if compiler.parser.hadError {
        return Result::Err(false);
    }
    return Result::Ok(compiler.chunk);
}



