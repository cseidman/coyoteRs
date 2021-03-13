use crate::rules::* ;
use crate::rules::Precedence::* ;
use crate::compiler::* ;

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum TokenType {
    // Single-character tokens.
    T_LEFT_PAREN, T_RIGHT_PAREN,
    T_LEFT_BRACE, T_RIGHT_BRACE,
    T_LEFT_BRACKET, T_RIGHT_BRACKET,
    T_COMMA, T_DOT, T_MINUS, T_PLUS,
    T_SEMICOLON, T_SLASH, T_STAR, T_COLON,
    T_DOUBLE_COLON,

    // One or two character tokens.
    T_BANG, T_BANG_EQUAL,
    T_EQUAL, T_EQUAL_EQUAL,
    T_GREATER, T_GREATER_EQUAL,
    T_LESS, T_LESS_EQUAL,

    // Literals.
    T_IDENTIFIER, T_STRING, T_INTEGER, T_DOUBLE, T_BOOL,

    // Keywords.
    T_AND, T_CLASS, T_ELSE, T_FALSE,
    T_FOR, T_FN, T_IF, T_NIL, T_OR,
    T_PRINT, T_RETURN, T_IMPORT, T_THIS,
    T_TRUE, T_LET, T_LOOP, T_MODULE,

    T_ERROR,
    T_EOF,
    T_START,
    T_CR
}

impl TokenType {

    pub fn get_rule(&self) -> ParseRule {
        match self {
            T_LEFT_PAREN => ParseRule{prefix: Some(Compiler::grouping), infix: None, prec: PREC_NONE},
            T_INTEGER    =>  ParseRule{prefix: Some(Compiler::integer), infix: None, prec: PREC_NONE} ,
            T_DOUBLE     =>  ParseRule{prefix: Some(Compiler::double), infix: None, prec: PREC_NONE} ,
            T_MINUS      =>  ParseRule{prefix: Some(Compiler::unary), infix: Some(Compiler::binary), prec: PREC_TERM} ,
            T_PLUS       =>  ParseRule{prefix: None, infix: Some(Compiler::binary), prec: PREC_TERM} ,
            T_STAR       =>  ParseRule{prefix: None, infix: Some(Compiler::binary), prec: PREC_FACTOR} ,
            T_SLASH      =>  ParseRule{prefix: None, infix: Some(Compiler::binary), prec: PREC_FACTOR} ,
            T_FALSE      =>  ParseRule{prefix: Some(Compiler::literal), infix: None, prec: PREC_NONE} ,
            T_TRUE       =>  ParseRule{prefix: Some(Compiler::literal), infix: None, prec: PREC_NONE} ,
            T_NIL        =>  ParseRule{prefix: Some(Compiler::literal), infix: None, prec: PREC_NONE} ,
            T_BANG       =>  ParseRule{prefix: Some(Compiler::unary), infix: None, prec: PREC_NONE} ,
            T_EQUAL_EQUAL   =>  ParseRule{prefix: None, infix: Some(Compiler::binary), prec: PREC_EQUALITY} ,
            T_GREATER       =>  ParseRule{prefix: None, infix: Some(Compiler::binary), prec: PREC_COMPARISON} ,
            T_GREATER_EQUAL =>  ParseRule{prefix: None, infix: Some(Compiler::binary), prec: PREC_COMPARISON},
            T_LESS          =>  ParseRule{prefix: None, infix: Some(Compiler::binary), prec: PREC_COMPARISON} ,
            T_LESS_EQUAL    =>  ParseRule{prefix: None, infix: Some(Compiler::binary), prec: PREC_COMPARISON} ,
            _ => ParseRule{prefix: None,infix: None, prec: Precedence::PREC_NONE
            }
        }
     }
}


use TokenType::* ;

pub static TOKEN_LIST: [TokenType;49] =
    [T_LEFT_PAREN, T_RIGHT_PAREN, T_LEFT_BRACE,
        T_RIGHT_BRACE, T_LEFT_BRACKET, T_RIGHT_BRACKET,
        T_COMMA, T_DOT, T_MINUS,
        T_PLUS, T_SEMICOLON, T_SLASH,
        T_STAR, T_COLON, T_BANG,
        T_BANG_EQUAL, T_EQUAL, T_EQUAL_EQUAL,
        T_GREATER, T_GREATER_EQUAL, T_LESS,
        T_LESS_EQUAL, T_DOUBLE_COLON, T_IDENTIFIER, T_STRING,
        T_DOUBLE, T_INTEGER, T_BOOL,
        T_AND, T_CLASS, T_ELSE,
        T_FALSE, T_FOR, T_FN,
        T_IF, T_NIL, T_OR,
        T_PRINT, T_RETURN, T_IMPORT,
        T_THIS, T_TRUE, T_LET,
        T_LOOP, T_MODULE, T_ERROR, T_EOF, T_START, T_CR] ;

macro_rules! SCANNER_NAME {
        ($scanner:expr) => {{
            let start = $scanner.start ;
            let current = $scanner.current ;
            $scanner.code[start..current].into_iter().collect()
        }}
    }

#[derive(PartialEq)]
#[derive(Clone)]
pub struct Token {
    pub name: String,
    pub toktype: TokenType,
    pub line: usize
}

pub struct Scanner {
    code: Vec<char> ,
    start: usize,
    current: usize,
    line: usize
}

pub fn newScanner(source: String) -> Scanner {
    return Scanner {
        code: source.chars().collect(),
        start: 0 ,
        current: 0,
        line: 1
    }
}

impl Scanner {
    fn advance(&mut self) -> char {
        self.current += 1;
        return self.code[self.current - 1];
    }

    fn cmatch(&mut self, expected: char) -> bool {
        if self.isAtEnd() {
            return false;
        }
        if self.code[self.current] != expected {
            return false;
        }
        self.current += 1;
        return true;
    }

    fn peek(&self) -> char {
        let current = self.current;
        return self.code[current];
    }

    fn peekNext(&self) -> char {
        let current = self.current - 1;
        return self.code[current];
    }

    pub fn scanToken(&mut self) -> Token {
        if self.isAtEnd() {
            return self.makeToken(T_EOF);
        }

        self.skipWhitespace();

        self.start = self.current;

        let c = self.advance();

        if c.is_alphabetic() || c == '_' {
            return self.identifier();
        }

        if c.is_ascii_digit() {
            return self.number();
        }

        return match c {
            '(' => self.makeToken(T_LEFT_PAREN),
            ')' => self.makeToken(T_RIGHT_PAREN),
            '{' => self.makeToken(T_LEFT_BRACE),
            '}' => self.makeToken(T_RIGHT_BRACE),
            ';' => self.makeToken(T_SEMICOLON),
            ',' => self.makeToken(T_COMMA),
            '.' => self.makeToken(T_DOT),
            '-' => self.makeToken(T_MINUS),
            '+' => self.makeToken(T_PLUS),
            '/' => self.makeToken(T_SLASH),
            '*' => self.makeToken(T_STAR),
            ':' => if self.cmatch(':') {
                    self.makeToken(T_DOUBLE_COLON)
                } else {
                    self.makeToken(T_COLON)
                },
            '!' =>
                if self.cmatch('=') {
                    self.makeToken(T_BANG_EQUAL)
                } else {
                    self.makeToken(T_BANG)
                } ,
            '=' => if self.cmatch('=') {
                self.makeToken(T_EQUAL_EQUAL)
            } else {
                self.makeToken(T_EQUAL)
            },
            '>' =>
                if self.cmatch('=') {
                    self.makeToken(T_GREATER_EQUAL)
                } else {
                    self.makeToken(T_GREATER)
                },
            '<' =>
                if self.cmatch('=') {
                    self.makeToken(T_LESS_EQUAL)
                } else {
                    self.makeToken(T_LESS)
                },
            '"' => return self.string(),
            '\n'=> self.makeToken(T_CR),
            _ => self.errorToken("Unexpected character".to_string())
        }
    }

    pub fn isAtEnd(&self) -> bool {
        return self.current >= self.code.len()-1 ;
    }

    pub fn makeToken(&self, tokType: TokenType) -> Token {
        return Token {
            name: SCANNER_NAME!(self),
            toktype: tokType,
            line: self.line,
        }
    }

    pub fn errorToken(&self, message: String) -> Token {
        return Token {
            name: message,
            toktype: T_ERROR,
            line: self.line,
        }
    }

    fn skipWhitespace(&mut self) {
        loop {
            if self.isAtEnd() {
                return ;
            }
            let c = self.peek();
            match c {
                '\r'
                | ' '
                | '\t' => { self.advance(); },
                '\n' => {
                    self.line += 1;
                    self.advance();
                },
                '/' => {
                    if self.peekNext() == '/' {
                        while self.peek() != '\n' && !self.isAtEnd() {
                            self.advance();
                        }
                    } else {
                        return;
                    }
                },
                _ => return
            }
        }
    }

    fn identifierType(&self,ident: &str) -> TokenType {
        return match ident {
            "and" => T_AND,
            "or" => T_OR,
            "if" => T_IF,
            "return" => T_RETURN,
            "let" => T_LET,
            "true" => T_TRUE,
            "for" => T_FOR,
            "loop" => T_LOOP,
            "import" => T_IMPORT,
            "class" => T_CLASS,
            "fn" => T_FN,
            "this" => T_THIS,
            "false" => T_FALSE,
            "nil" => T_NIL,
            "else" => T_ELSE,
            "bool" => T_BOOL,
            _ => T_IDENTIFIER
        }
    }

    fn identifier(&mut self) -> Token {
        while self.peek().is_alphanumeric() || self.peek() == '_' {
            self.advance();
        }
        let ident: String = SCANNER_NAME!(self);
        return self.makeToken( self.identifierType(ident.as_str()));
    }

    fn number(&mut self) -> Token {
        while self.peek().is_ascii_digit() {
            self.advance();
        }

        if self.peek() == '.' && self.peekNext().is_ascii_digit() {
            self.advance();
            while self.peek().is_ascii_digit() {
                self.advance();
            }
            return self.makeToken( T_DOUBLE);
        }
        return self.makeToken( T_INTEGER);
    }

    fn string(&mut self) -> Token {
        while self.peek() != '"' && !self.isAtEnd() {
            if self.peek() == '\n' {
                self.line += 1;
                self.advance();
            }
        }
        if self.isAtEnd() {
            return self.errorToken("Unterminated string".to_string());
        }

        self.advance();
        return self.makeToken( T_STRING);
    }
}