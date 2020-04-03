#![allow(non_snake_case)]

use num_traits::FromPrimitive;

use super::chunk::{Chunk, OpCode};
use super::value::printValue;

pub fn disassembleChunk(chunk: &Chunk, name: &str) {
    println!("== {} ==", name);

    let mut offset = 0;
    while offset < chunk.code.len() {
        offset = disassembleInstruction(chunk, offset);
    }
}

pub fn disassembleInstruction(chunk: &Chunk, offset: usize) -> usize {
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
        OpCode::NIL => {
            simpleInstruction("OP_NIL", offset)
        }
        OpCode::TRUE => {
            simpleInstruction("OP_TRUE", offset)
        }
        OpCode::FALSE => {
            simpleInstruction("OP_FALSE", offset)
        }
        OpCode::EQUAL => {
            simpleInstruction("OP_EQUAL", offset)
        }
        OpCode::GREATER => {
            simpleInstruction("OP_GREATER", offset)
        }
        OpCode::LESS => {
            simpleInstruction("OP_LESS", offset)
        }
        OpCode::ADD => {
            simpleInstruction("OP_ADD", offset)
        }
        OpCode::SUBTRACT => {
            simpleInstruction("OP_SUBTRACT", offset)
        }
        OpCode::MULTIPLY => {
            simpleInstruction("OP_MULTIPLY", offset)
        }
        OpCode::DIVIDE => {
            simpleInstruction("OP_DIVIDE", offset)
        }
        OpCode::NOT => {
            simpleInstruction("OP_NOT", offset)
        }
        OpCode::NEGATE => {
            simpleInstruction("OP_NEGATE", offset)
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
    printValue(&chunk.constants.values[constant as usize]);
    println!("'");
    offset + 2
}
