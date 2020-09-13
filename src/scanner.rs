#[derive(PartialEq)]
#[derive(Copy, Clone)]
pub enum TokenType {
    // Single-character tokens.
    T_LEFT_PAREN, T_RIGHT_PAREN,
    T_LEFT_BRACE, T_RIGHT_BRACE,
    T_COMMA, T_DOT, T_MINUS, T_PLUS,
    T_SEMICOLON, T_SLASH, T_STAR,

    // One or two character tokens.
    T_BANG, T_BANG_EQUAL,
    T_EQUAL, T_EQUAL_EQUAL,
    T_GREATER, T_GREATER_EQUAL,
    T_LESS, T_LESS_EQUAL,

    // Literals.
    T_IDENTIFIER, T_STRING, T_INTEGER, T_DOUBLE,

    // Keywords.
    T_AND, T_CLASS, T_ELSE, T_FALSE,
    T_FOR, T_FN, T_IF, T_NIL, T_OR,
    T_PRINT, T_RETURN, T_SUPER, T_THIS,
    T_TRUE, T_LET, T_WHILE,

    T_ERROR,
    T_EOF,
    T_START
}

use TokenType::* ;

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
        if self.isAtEnd() {
            return '\0';
        }
        let current = self.current + 1;
        return self.code[current];
    }

    pub fn scanToken(&mut self) -> Token {
        self.skipWhitespace();

        self.start = self.current;
        if self.isAtEnd() {
            return self.makeToken(T_EOF);
        }

        let c = self.advance();

        if c.is_alphabetic() || c == '_' {
            return self.identifier();
        }

        if c.is_numeric() {
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
            '!' =>
                if self.cmatch('=') {
                    self.makeToken(T_BANG_EQUAL)
                } else {
                    self.makeToken(T_BANG)
                }
            ,
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
            _ => self.errorToken("Unexpected character".to_string())
        }
    }

    pub fn isAtEnd(&self) -> bool {
        return self.code[self.current] == '\0';
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
            "while" => T_WHILE,
            "super" => T_SUPER,
            "class" => T_CLASS,
            "fn" => T_FN,
            "this" => T_THIS,
            "false" => T_FALSE,
            "nil" => T_NIL,
            "else" => T_ELSE,
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
        while self.peek().is_numeric() {
            self.advance();
        }

        if self.peek() == '.' && self.peekNext().is_numeric() {
            self.advance();
            while self.peek().is_numeric() {
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