mod chunk;
mod debug;
mod value;
mod vm;

use self::chunk::{OpCode, initChunk, writeChunk, addConstant};
use self::debug::disassembleChunk;
use self::vm::{/*initVM,*/ interpret};

fn main() {
    let mut chunk = initChunk();
    let constant = addConstant(&mut chunk, 1.2);
    writeChunk(&mut chunk, OpCode::CONSTANT as u8, 123);
    writeChunk(&mut chunk, constant as u8, 123);
    writeChunk(&mut chunk, OpCode::RETURN as u8, 123);
    disassembleChunk(&mut chunk, "test chunk");
    interpret(&chunk);
}