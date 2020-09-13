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

fn advance(scanner: &mut Scanner) -> char {
    scanner.current+1 ;
    return scanner.code[scanner.current-1] ;
}

fn cmatch(scanner: &mut Scanner, expected: char) -> bool {
    if isAtEnd(scanner) {
        return false ;
    }
    if scanner.code[scanner.current] != expected {
        return false ;
    }
    scanner.current += 1 ;
    return true ;
}

fn peek(scanner: &Scanner) -> char {
    let current = scanner.current ;
    return scanner.code[current] ;
}

fn peekNext(scanner: &Scanner) -> char {
    if isAtEnd(scanner) {
        return '\0' ;
    }
    let current = scanner.current+1 ;
    return scanner.code[current] ;
}

pub fn scanToken(scanner: &mut Scanner) -> Token {

    skipWhitespace(scanner);

    scanner.start = scanner.current  ;
    if isAtEnd(scanner) {
        return makeToken(scanner,T_EOF) ;
    }

    let c = advance(scanner);

    if c.is_alphabetic() || c == '_' {
        return identifier(scanner) ;
    }

    if c.is_numeric() {
        return number(scanner) ;
    }

    return match c {
        '(' => makeToken(scanner, T_LEFT_PAREN),
        ')' => makeToken(scanner, T_RIGHT_PAREN),
        '{' => makeToken(scanner, T_LEFT_BRACE),
        '}' => makeToken(scanner, T_RIGHT_BRACE),
        ';' => makeToken(scanner, T_SEMICOLON),
        ',' => makeToken(scanner, T_COMMA),
        '.' => makeToken(scanner, T_DOT),
        '-' => makeToken(scanner, T_MINUS),
        '+' => makeToken(scanner, T_PLUS),
        '/' => makeToken(scanner, T_SLASH),
        '*' => makeToken(scanner, T_STAR),
        '!' =>
            if cmatch(scanner, '=') {
                makeToken(scanner,T_BANG_EQUAL)
            } else {
                makeToken(scanner, T_BANG)
            }
        ,
        '=' => if cmatch(scanner, '=') {
                makeToken(scanner, T_EQUAL_EQUAL)
            }  else {
                makeToken(scanner, T_EQUAL)
            },
        '>' =>
            if cmatch(scanner, '=') {
                makeToken(scanner, T_GREATER_EQUAL)
            } else {
                makeToken(scanner, T_GREATER)
            },
        '<' =>
            if cmatch(scanner, '=') {
                makeToken(scanner, T_LESS_EQUAL)
            } else {
                makeToken(scanner, T_LESS)
            },
        '"' => return string(scanner),
        _   => errorToken(scanner, "Unexpected character".to_string())
    }

}

pub fn isAtEnd(scanner: &Scanner) -> bool {
    return scanner.code[scanner.current] == '\0' ;
}

pub fn makeToken(scanner: &Scanner, tokType: TokenType) -> Token {

    return Token {
        name: SCANNER_NAME!(scanner),
        toktype: tokType,
        line: scanner.line,
    }
}

pub fn errorToken(scanner: &Scanner, message: String) -> Token {
    return Token {
        name: message,
        toktype: T_ERROR,
        line: scanner.line,
    }
}

fn skipWhitespace(scanner: &mut Scanner) {
    loop {
        let c = peek(scanner) ;
        match c {
              '\r'
            | ' '
            | '\t'  => {advance(scanner);},
            '\n'    => {
                scanner.line+=1 ;
                advance(scanner) ;
            },
            '/'     => {
                if peekNext(scanner) == '/' {
                    while peek(scanner) != '\n' && !isAtEnd(scanner) {
                        advance(scanner) ;
                    }
                } else {
                    return ;
                }
            },
            _       => return
        }
    }
}

fn identifierType(ident: &str) -> TokenType {

    return match ident {
        "and"       => T_AND,
        "or"        => T_OR,
        "if"        => T_IF,
        "return"    => T_RETURN,
        "let"       => T_LET,
        "true"      => T_TRUE,
        "for"       => T_FOR,
        "while"     => T_WHILE,
        "super"     => T_SUPER,
        "class"     => T_CLASS,
        "fn"        => T_FN,
        "this"      => T_THIS,
        "false"     => T_FALSE,
        "nil"       => T_NIL,
        "else"      => T_ELSE,
        _ => T_IDENTIFIER
    }
}

fn identifier(scanner: &mut Scanner) -> Token {
    while peek(scanner).is_alphanumeric() || peek(scanner) == '_'  {
        advance(scanner);
    }
    let ident:String = SCANNER_NAME!(scanner) ;
    return makeToken(scanner, identifierType(ident.as_str())) ;
}

fn number(scanner: &mut Scanner) -> Token {
    while peek(scanner).is_numeric() {
        advance(scanner) ;
    }

    if peek(scanner) == '.' && peekNext(scanner).is_numeric() {
        advance(scanner) ;
        while peek(scanner).is_numeric() {
            advance(scanner) ;
        }
        return makeToken(scanner, T_DOUBLE) ;
    }

    return makeToken(scanner, T_INTEGER) ;
}

fn string(scanner: &mut Scanner) -> Token{
    while peek(scanner) != '"' && !isAtEnd(scanner) {
        if peek(scanner) == '\n' {
            scanner.line += 1;
            advance(scanner);
        }
    }
   if isAtEnd(scanner) {
    return errorToken(scanner,"Unterminated string".to_string()) ;
   }

   advance(scanner) ;
   return makeToken(scanner,T_STRING) ;

}