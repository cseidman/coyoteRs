use crate::scanner::* ;
use crate::chunk::* ;

use std::io::{self, Write};
use std::borrow::Borrow;

#[derive(Clone)]
struct Parser {
    current: Token,
    previous: Token,
    hadError: bool
}

fn advance(parser: &mut Parser, scanner: &mut Scanner) {

    use TokenType::* ;
    parser.previous = parser.current.clone() ;

    loop {
        parser.current = scanToken(scanner);
        if parser.current.toktype != T_ERROR {
            break ;
        }
        errorAtCurrent(parser);
    }
}

pub fn compile(source: String, chunk: &mut Chunk) -> bool {
    use TokenType::* ;
    let mut scanner = newScanner(source) ;
    let mut parser = Parser {
        current: makeToken(&scanner,T_START),
        previous: makeToken(&scanner,T_START),
        hadError: false
    };
    advance(&mut parser, &mut scanner) ;
    expression() ;
    consume(T_EOF, "Expect end of expression") ;

    return true ;
}

fn errorAtCurrent(parser: &mut Parser) {
    errorAt(parser, &parser.current.name);
}

fn errorAt (parser: &mut Parser, message: &str) {
    use TokenType::* ;
    io::stderr().write_all(b"[line %d] Error", &parser.current.line);

    if parser.current.toktype == T_EOF {
        io::stderr().write_all(b" at end");
    } else if parser.current.toktype == T_ERROR {
        // Nothing.
    } else {
        io::stderr().write_all(b" at '%.*s'", &parser.current.name);
    }

    io::stderr().write_all(b": %s\n", message);
    parser.hadError = true;
}