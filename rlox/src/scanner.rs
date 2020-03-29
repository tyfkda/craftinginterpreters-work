#![allow(non_camel_case_types, non_snake_case)]

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TokenType {
  // Single-character tokens.
  LEFT_PAREN, RIGHT_PAREN,
  LEFT_BRACE, RIGHT_BRACE,
  COMMA, DOT, MINUS, PLUS,
  SEMICOLON, SLASH, STAR,

  // One or two character tokens.
  BANG, BANG_EQUAL,
  EQUAL, EQUAL_EQUAL,
  GREATER, GREATER_EQUAL,
  LESS, LESS_EQUAL,

  // Literals.
  IDENTIFIER, STRING, NUMBER,

  // Keywords.
  AND, CLASS, ELSE, FALSE,
  FOR, FUN, IF, NIL, OR,
  PRINT, RETURN, SUPER, THIS,
  TRUE, VAR, WHILE,

  ERROR,
  EOF,
}

pub struct Scanner<'a> {
    start: &'a str,
    current: usize,
    line: i32,
}

pub struct Token<'a> {
    pub token_type: TokenType,
    pub start: &'a str,
    pub line: i32,
}

pub fn initScanner<'a>(source: &'a str) -> Scanner<'a> {
    Scanner {
        start: source,
        current: 0,
        line: 1,
    }
}

fn isAlpha(c: char) -> bool {
    (c >= 'a' && c <= 'z') ||
        (c >= 'A' && c <= 'Z') ||
        c == '_'
}

fn isDigit(c: char) -> bool {
    c >= '0' && c <= '9'
}

pub fn scanToken<'a>(scanner: &'a mut Scanner) -> Token<'a> {
    skipWhitespace(scanner);

    scanner.start = &scanner.start[scanner.current..scanner.start.len()];
    scanner.current = 0;

    if isAtEnd(scanner) {
        return makeToken(scanner, TokenType::EOF);
    }

    let c = advance(scanner);
    if isAlpha(c) { return scanIdentifier(scanner); }
    if isDigit(c) { return scanNumber(scanner); }

    match c {
        '(' => { return makeToken(scanner, TokenType::LEFT_PAREN); }
        ')' => { return makeToken(scanner, TokenType::RIGHT_PAREN); }
        '{' => { return makeToken(scanner, TokenType::LEFT_BRACE); }
        '}' => { return makeToken(scanner, TokenType::RIGHT_BRACE); }
        ';' => { return makeToken(scanner, TokenType::SEMICOLON); }
        ',' => { return makeToken(scanner, TokenType::COMMA); }
        '.' => { return makeToken(scanner, TokenType::DOT); }
        '-' => { return makeToken(scanner, TokenType::MINUS); }
        '+' => { return makeToken(scanner, TokenType::PLUS); }
        '/' => { return makeToken(scanner, TokenType::SLASH); }
        '*' => { return makeToken(scanner, TokenType::STAR); }
        '!' => {
            let token_type = if matchChar(scanner, '=') { TokenType::BANG_EQUAL } else { TokenType::BANG };
            return makeToken(scanner, token_type);
        }
        '=' => {
            let token_type = if matchChar(scanner, '=') { TokenType::EQUAL_EQUAL } else { TokenType::EQUAL };
            return makeToken(scanner, token_type);
        }
        '<' => {
            let token_type = if matchChar(scanner, '=') { TokenType::LESS_EQUAL } else { TokenType::LESS };
            return makeToken(scanner, token_type);
        }
        '>' => {
            let token_type = if matchChar(scanner, '=') { TokenType::GREATER_EQUAL } else { TokenType::GREATER };
            return makeToken(scanner, token_type);
        }
        '"' => {
            return scanString(scanner);
        }
        _ => {}
    }

    return errorToken(scanner, "Unexpected character.");
}

fn peek(scanner: &Scanner) -> char {
    scanner.start[scanner.current..scanner.current + 1].chars().next().unwrap()
}

fn isAtEnd(scanner: &Scanner) -> bool {
    peek(scanner) == '\0'
}

fn peekNext(scanner: &Scanner) -> char {
    if isAtEnd(scanner) {
        '\0'
    } else {
        scanner.start[1..2].chars().next().unwrap()
    }
}

fn advance(scanner: &mut Scanner) -> char {
    let c = peek(scanner);
    scanner.current += 1;
    c
}

fn matchChar(scanner: &mut Scanner, expected: char) -> bool {
    if isAtEnd(scanner)
        || scanner.start[scanner.current..scanner.current + 1].chars().next().unwrap() != expected
    {
        false
    } else {
        scanner.current += 1;
        true
    }
}

fn skipWhitespace(scanner: &mut Scanner) {
    loop {
        let c = peek(scanner);
        match c {
            ' ' | '\r' | '\t' => {
                advance(scanner);
            }
            '\n' => {
                scanner.line += 1;
                advance(scanner);
            }
            '/' => {
                if peekNext(scanner) == '/' {
                    // A comment goes until the end of the line.
                    while peek(scanner) != '\n' && !isAtEnd(scanner) {
                        advance(scanner);
                    }
                } else {
                    return;
                }
            }
            _ => { return; }
        }
    }
}

fn scanNumber<'a>(scanner: &'a mut Scanner) -> Token<'a> {
    while isDigit(peek(scanner)) {
        advance(scanner);
    }

    if peek(scanner) == '.' && isDigit(peekNext(scanner)) {
        // Consume the ".".
        advance(scanner);

        while isDigit(peek(scanner)) {
            advance(scanner);
        }
    }

    makeToken(scanner, TokenType::NUMBER)
}

fn checkKeyword(scanner: &Scanner, start: usize, rest: &str, token_type: TokenType) -> TokenType {
    if scanner.current == start + rest.len() &&
        scanner.start[start..scanner.current].eq(rest)
    {
        token_type
    } else {
        TokenType::IDENTIFIER
    }
}

fn identifierType(scanner: &Scanner) -> TokenType {
    match scanner.start.chars().next().unwrap() {
        'a' => { return checkKeyword(scanner, 1, "nd", TokenType::AND); }
        'c' => { return checkKeyword(scanner, 1, "lass", TokenType::CLASS); }
        'e' => { return checkKeyword(scanner, 1, "lse", TokenType::ELSE); }
        'f' => {
            if scanner.current > 1 {
                match scanner.start[1..2].chars().next().unwrap() {
                    'a' => { return checkKeyword(scanner, 2, "lse", TokenType::FALSE); }
                    'o' => { return checkKeyword(scanner, 2, "r", TokenType::FOR); }
                    'u' => { return checkKeyword(scanner, 2, "n", TokenType::FUN); }
                    _ => {}
                }
            }
        }
        'i' => { return checkKeyword(scanner, 1, "f", TokenType::IF); }
        'n' => { return checkKeyword(scanner, 1, "il", TokenType::NIL); }
        'o' => { return checkKeyword(scanner, 1, "r", TokenType::OR); }
        'p' => { return checkKeyword(scanner, 1, "rint", TokenType::PRINT); }
        'r' => { return checkKeyword(scanner, 1, "eturn", TokenType::RETURN); }
        's' => { return checkKeyword(scanner, 1, "uper", TokenType::SUPER); }
        't' => {
            if scanner.current > 1 {
                match scanner.start[1..2].chars().next().unwrap() {
                    'h' => { return checkKeyword(scanner, 2, "is", TokenType::THIS); }
                    'r' => { return checkKeyword(scanner, 2, "ue", TokenType::TRUE); }
                    _ => {}
                }
            }
        }
        'v' => { return checkKeyword(scanner, 1, "ar", TokenType::VAR); }
        'w' => { return checkKeyword(scanner, 1, "hile", TokenType::WHILE); }
        _ => {}
    }
    TokenType::IDENTIFIER
}

fn scanIdentifier<'a>(scanner: &'a mut Scanner) -> Token<'a> {
    while isAlpha(peek(scanner)) || isDigit(peek(scanner)) {
        advance(scanner);
    }

    let idType = identifierType(scanner);
    makeToken(scanner, idType)
}

fn scanString<'a>(scanner: &'a mut Scanner) -> Token<'a> {
    while peek(scanner) != '"' && !isAtEnd(scanner) {
        if peek(scanner) == '\n' { scanner.line += 1; }
        advance(scanner);
    }

    if isAtEnd(scanner) {
        return errorToken(scanner, "Unterminated string.");
    }

    // The closing quote.
    advance(scanner);
    makeToken(scanner, TokenType::STRING)
}

fn makeToken<'a>(scanner: &'a Scanner, token_type: TokenType) -> Token<'a> {
    Token {
        token_type,
        start: &scanner.start[0..scanner.current],
        line: scanner.line,
    }
}

fn errorToken<'a>(scanner: &Scanner, message: &'a str) -> Token<'a> {
    Token {
        token_type: TokenType::ERROR,
        start: message,
        line: scanner.line,
    }
}
