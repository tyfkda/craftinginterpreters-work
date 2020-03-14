#![allow(non_snake_case)]

use num_derive::FromPrimitive;

use super::value::{Value, ValueArray, initValueArray, writeValueArray};

#[derive(FromPrimitive)]
pub enum OpCode {
    CONSTANT,
    RETURN,
}

pub struct Chunk {
    pub code: Vec<u8>,
    pub lines: Vec<u32>,
    pub constants: ValueArray,
}

pub fn initChunk() -> Chunk {
    Chunk {
        code: vec![],
        lines: vec![],
        constants: initValueArray(),
    }
}

pub fn writeChunk(chunk: &mut Chunk, byte: u8, line: u32) {
    chunk.code.push(byte);
    chunk.lines.push(line);
}

pub fn addConstant(chunk: &mut Chunk, value: Value) -> usize {
    writeValueArray(&mut chunk.constants, value);
    chunk.constants.count() - 1
}
