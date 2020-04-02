#![allow(non_upper_case_globals)]

use num_derive::FromPrimitive;
use num_traits::FromPrimitive;

use super::chunk::{addConstant, writeChunk, Chunk, OpCode};
use super::debug::disassembleChunk;
use super::scanner::{initScanner, makeToken, scanToken, Scanner, Token, TokenType};
use super::value::Value;

struct Parser<'a> {
    scanner: Scanner<'a>,
    chunk: &'a mut Chunk,
    current: Token<'a>,
    previous: Token<'a>,
    hadError: bool,
    panicMode: bool,
}

#[derive(Clone, Copy, Debug, FromPrimitive)]
pub enum Precedence {
  NONE,
  ASSIGNMENT,  // =
  OR,          // or
  AND,         // and
  EQUALITY,    // == !=
  COMPARISON,  // < > <= >=
  TERM,        // + -
  FACTOR,      // * /
  UNARY,       // ! -
  CALL,        // . ()
  PRIMARY,
}

struct ParseRule {
    prefix: Option<fn(&mut Parser)>,
    infix: Option<fn(&mut Parser)>,
    precedence: Precedence,
}

fn errorAt(parser: &mut Parser, token: &Token, message: &str) {
    if parser.panicMode { return; }
    parser.panicMode = true;

    eprint!("[line {}] Error", token.line);

    if token.token_type == TokenType::EOF {
        eprint!(" at end");
    } else if token.token_type == TokenType::ERROR {
        // Nothing.
    } else {
        eprint!(" at '{}'", token.start);
    }

    eprintln!(": {}", message);
    parser.hadError = true;
}

fn error(parser: &mut Parser, message: &str) {
    let token = parser.previous.clone();
    errorAt(parser, &token, message);
}

fn errorAtCurrent(parser: &mut Parser, message: &str) {
    let token = parser.current.clone();
    errorAt(parser, &token, message);
}

fn advance<'a>(parser: &mut Parser<'a>) {
    parser.previous = parser.current.clone();

    //loop {
        parser.current = scanToken(&mut parser.scanner);
        if parser.current.token_type != TokenType::ERROR {
            //break;
            return;
        }
        errorAtCurrent(parser, parser.current.start);
    //}
}

fn consume<'a>(parser: &mut Parser<'a>, token_type: TokenType, message: &str) {
    if parser.current.token_type == token_type {
        advance(parser);
        return;
    }

    errorAtCurrent(parser, message);
}

fn emitByte<'a>(parser: &mut Parser<'a>, byte: u8) {
    writeChunk(&mut parser.chunk, byte, parser.previous.line);
}

fn emitBytes<'a>(parser: &mut Parser<'a>, byte1: u8, byte2: u8) {
    emitByte(parser, byte1);
    emitByte(parser, byte2);
}

fn emitReturn<'a>(parser: &mut Parser<'a>) {
    emitByte(parser, OpCode::RETURN as u8);
}

fn makeConstant<'a>(parser: &mut Parser<'a>, value: Value) -> u8 {
    let constant = addConstant(parser.chunk, value);
    if constant > u8::MAX as usize {
        error(parser, "Too many constants in one chunk.");
        return 0;
    }
    return constant as u8;
}

fn emitConstant<'a>(parser: &mut Parser<'a>, value: Value) {
    let i = makeConstant(parser, value);
    emitBytes(parser, OpCode::CONSTANT as u8, i);
}

fn endCompiler<'a>(parser: &mut Parser<'a>) {
    emitReturn(parser);

    if !parser.hadError {
        disassembleChunk(parser.chunk, "code");
    }
}

fn binary<'a>(parser: &mut Parser<'a>) {
    // Remember the operator.
    let operatorType = parser.previous.token_type;

    // Compile the right operand.
    let rule = getRule(operatorType);
    parsePrecedence(parser, FromPrimitive::from_u8(rule.precedence as u8 + 1).unwrap());

    // Emit the operator instruction.
    match operatorType {
        TokenType::PLUS   => { emitByte(parser, OpCode::ADD as u8); }
        TokenType::MINUS  => { emitByte(parser, OpCode::SUBTRACT as u8); }
        TokenType::STAR   => { emitByte(parser, OpCode::MULTIPLY as u8); }
        TokenType::SLASH  => { emitByte(parser, OpCode::DIVIDE as u8); }
        _ => { return; }
    }
}

fn literal<'a>(parser: &mut Parser<'a>) {
    match parser.previous.token_type {
        TokenType::FALSE => { emitByte(parser, OpCode::FALSE as u8); }
        TokenType::NIL   => { emitByte(parser, OpCode::NIL as u8); }
        TokenType::TRUE  => { emitByte(parser, OpCode::TRUE as u8); }
        _ => { return; }
    }
}

fn grouping<'a>(parser: &mut Parser<'a>) {
    expression(parser);
    consume(parser, TokenType::RIGHT_PAREN, "Expect ')' after expression.");
}

fn number<'a>(parser: &mut Parser<'a>) {
    let value = parser.previous.start.parse::<f64>().unwrap();
    emitConstant(parser, Value::NUMBER(value));
}

fn unary<'a>(parser: &mut Parser<'a>) {
    let operatorType = parser.previous.token_type;

    // Compile the operand.
    parsePrecedence(parser, Precedence::UNARY);

    match operatorType {
        TokenType::BANG  => { emitByte(parser, OpCode::NOT as u8); }
        TokenType::MINUS => { emitByte(parser, OpCode::NEGATE as u8); }
        _ => { return; }
    }
}

const rules: [ParseRule; 40] = [
    ParseRule { prefix: Some(grouping), infix: None,          precedence: Precedence::NONE },       // TOKEN_LEFT_PAREN
    ParseRule { prefix: None,           infix: None,          precedence: Precedence::NONE },       // TOKEN_RIGHT_PAREN
    ParseRule { prefix: None,           infix: None,          precedence: Precedence::NONE },       // TOKEN_LEFT_BRACE
    ParseRule { prefix: None,           infix: None,          precedence: Precedence::NONE },       // TOKEN_RIGHT_BRACE
    ParseRule { prefix: None,           infix: None,          precedence: Precedence::NONE },       // TOKEN_COMMA
    ParseRule { prefix: None,           infix: None,          precedence: Precedence::NONE },       // TOKEN_DOT
    ParseRule { prefix: Some(unary),    infix: Some(binary),  precedence: Precedence::TERM },       // TOKEN_MINUS
    ParseRule { prefix: None,           infix: Some(binary),  precedence: Precedence::TERM },       // TOKEN_PLUS
    ParseRule { prefix: None,           infix: None,          precedence: Precedence::NONE },       // TOKEN_SEMICOLON
    ParseRule { prefix: None,           infix: Some(binary),  precedence: Precedence::FACTOR },     // TOKEN_SLASH
    ParseRule { prefix: None,           infix: Some(binary),  precedence: Precedence::FACTOR },     // TOKEN_STAR
    ParseRule { prefix: Some(unary),    infix: None,          precedence: Precedence::NONE },       // TOKEN_BANG
    ParseRule { prefix: None,           infix: None,          precedence: Precedence::NONE },       // TOKEN_BANG_EQUAL
    ParseRule { prefix: None,           infix: None,          precedence: Precedence::NONE },       // TOKEN_EQUAL
    ParseRule { prefix: None,           infix: None,          precedence: Precedence::NONE },       // TOKEN_EQUAL_EQUAL
    ParseRule { prefix: None,           infix: None,          precedence: Precedence::NONE },       // TOKEN_GREATER
    ParseRule { prefix: None,           infix: None,          precedence: Precedence::NONE },       // TOKEN_GREATER_EQUAL
    ParseRule { prefix: None,           infix: None,          precedence: Precedence::NONE },       // TOKEN_LESS
    ParseRule { prefix: None,           infix: None,          precedence: Precedence::NONE },       // TOKEN_LESS_EQUAL
    ParseRule { prefix: None,           infix: None,          precedence: Precedence::NONE },       // TOKEN_IDENTIFIER
    ParseRule { prefix: None,           infix: None,          precedence: Precedence::NONE },       // TOKEN_STRING
    ParseRule { prefix: Some(number),   infix: None,          precedence: Precedence::NONE },       // TOKEN_NUMBER
    ParseRule { prefix: None,           infix: None,          precedence: Precedence::NONE },       // TOKEN_AND
    ParseRule { prefix: None,           infix: None,          precedence: Precedence::NONE },       // TOKEN_CLASS
    ParseRule { prefix: None,           infix: None,          precedence: Precedence::NONE },       // TOKEN_ELSE
    ParseRule { prefix: Some(literal),  infix: None,          precedence: Precedence::NONE },       // TOKEN_FALSE
    ParseRule { prefix: None,           infix: None,          precedence: Precedence::NONE },       // TOKEN_FOR
    ParseRule { prefix: None,           infix: None,          precedence: Precedence::NONE },       // TOKEN_FUN
    ParseRule { prefix: None,           infix: None,          precedence: Precedence::NONE },       // TOKEN_IF
    ParseRule { prefix: Some(literal),  infix: None,          precedence: Precedence::NONE },       // TOKEN_NIL
    ParseRule { prefix: None,           infix: None,          precedence: Precedence::NONE },       // TOKEN_OR
    ParseRule { prefix: None,           infix: None,          precedence: Precedence::NONE },       // TOKEN_PRINT
    ParseRule { prefix: None,           infix: None,          precedence: Precedence::NONE },       // TOKEN_RETURN
    ParseRule { prefix: None,           infix: None,          precedence: Precedence::NONE },       // TOKEN_SUPER
    ParseRule { prefix: None,           infix: None,          precedence: Precedence::NONE },       // TOKEN_THIS
    ParseRule { prefix: Some(literal),  infix: None,          precedence: Precedence::NONE },       // TOKEN_TRUE
    ParseRule { prefix: None,           infix: None,          precedence: Precedence::NONE },       // TOKEN_VAR
    ParseRule { prefix: None,           infix: None,          precedence: Precedence::NONE },       // TOKEN_WHILE
    ParseRule { prefix: None,           infix: None,          precedence: Precedence::NONE },       // TOKEN_ERROR
    ParseRule { prefix: None,           infix: None,          precedence: Precedence::NONE },       // TOKEN_EOF
];

fn getRule(token_type: TokenType) -> &'static ParseRule {
    &rules[token_type as usize]
}

fn parsePrecedence<'a>(parser: &mut Parser<'a>, precedence: Precedence) {
    advance(parser);
    let prefixRule = getRule(parser.previous.token_type).prefix;
    if let Some(prefixRule) = prefixRule {
        prefixRule(parser);

        while precedence as u8 <= getRule(parser.current.token_type).precedence as u8 {
            advance(parser);
            let infixRule = getRule(parser.previous.token_type).infix;
            if let Some(infixRule) = infixRule {
                infixRule(parser);
            }
        }
    } else {
        error(parser, "Expect expression.");
    }
}

fn expression<'a>(parser: &mut Parser<'a>) {
    parsePrecedence(parser, Precedence::ASSIGNMENT);
}

pub fn compile<'a>(source: &'a str, chunk: &mut Chunk) -> bool {
    let mut scanner = initScanner(source);
    let eof = makeToken(&mut scanner, TokenType::EOF);
    let mut parser = Parser {
        scanner,
        chunk,
        current: eof.clone(),
        previous: eof.clone(),
        hadError: false,
        panicMode: false,
    };

    advance(&mut parser);
    expression(&mut parser);
    consume(&mut parser, TokenType::EOF, "Expect end of expression.");
    endCompiler(&mut parser);
    !parser.hadError
}
