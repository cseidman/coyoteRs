use crate::scanner::* ;
use crate::chunk::* ;

use std::io::{self, Write};
use std::borrow::Borrow;

#[derive(Clone)]
struct Parser {
    current: Token,
    previous: Token,
    hadError: bool,
    panicMode: bool
}

struct Compiler {
    scanner: Scanner,
    parser: Parser,
    chunk: Chunk
}

impl Compiler {
    fn advance(&mut self) {
        use TokenType::*;
        self.parser.previous = self.parser.current.clone();

        loop {
            self.parser.current = self.scanner.scanToken();
            if self.parser.current.toktype != T_ERROR {
                break;
            }

            let mut msg = self.parser.current.name.clone();
            self.errorAtCurrent(msg.as_str());
        }
    }
    fn consume(&mut self, toktype: TokenType, message: &str) {
        if self.parser.current.toktype == toktype {
            self.advance();
            return;
        }

        self.errorAtCurrent(message);
    }
    fn expression(&mut self) {

    }

    fn errorAtCurrent(&mut self, message: &str) {
        self.errorAt(message);
    }

    fn errorAt(&mut self,message: &str) {

        if self.parser.panicMode {return;}
        self.parser.panicMode = true;

        use TokenType::* ;
        io::stderr().write_all(format!("[line {}] Error", &self.parser.current.line).as_bytes());

        if self.parser.current.toktype == T_EOF {
            io::stderr().write_all(b" at end");
        } else if self.parser.current.toktype == T_ERROR {
            // Nothing.
        } else {
            io::stderr().write_all(format!(" at '{}'", &self.parser.current.name).as_bytes());
        }

        io::stderr().write_all( format!(": {}\n", message).as_bytes());
        self.parser.hadError = true;
    }

    fn emitByte(&mut self, byte: u8) {
        self.chunk.writeChunk(byte, self.parser.previous.line);
    }

}

pub fn compile(source: String, chunk: Chunk) -> bool {

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
        chunk
    } ;

    use TokenType::* ;

    compiler.advance() ;
    compiler.expression() ;
    compiler.consume(T_EOF, "Expect end of expression") ;

    return !compiler.parser.hadError;
}

