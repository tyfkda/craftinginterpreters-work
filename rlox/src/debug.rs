#![allow(non_snake_case)]

use num_traits::FromPrimitive;

use super::chunk::{Chunk, OpCode};
use super::value::Value;

pub fn disassembleChunk(chunk: &Chunk, name: &str) {
    println!("== {} ==", name);

    let mut offset = 0;
    while offset < chunk.code.len() {
        offset = disassembleInstruction(chunk, offset);
    }
}

fn disassembleInstruction(chunk: &Chunk, offset: usize) -> usize {
    print!("{:04} ", offset);
    if offset > 0 && chunk.lines[offset] == chunk.lines[offset - 1] {
        print!("   | ");
    } else {
        print!("{:4} ", chunk.lines[offset]);
    }

    let instruction = FromPrimitive::from_u8(chunk.code[offset]).unwrap();
    match instruction {
        OpCode::CONSTANT => {
            constantInstruction("OP_CONSTANT", chunk, offset)
        }
        OpCode::RETURN => {
            simpleInstruction("OP_RETURN", offset)
        }
    }
}

fn simpleInstruction(name: &str, offset: usize) -> usize {
    println!("{}", name);
    offset + 1
}

fn constantInstruction(name: &str, chunk: &Chunk, offset: usize) -> usize {
    let constant = chunk.code[offset + 1];
    print!("{:-16} {:4} '", name, constant);
    printValue(chunk.constants.values[constant as usize]);
    println!("'");
    offset + 2
}

fn printValue(value: Value) {
    print!("{}", value);
}
