//use std::process;

mod chunk;
mod debug;
mod value;
mod vm;

use self::chunk::{OpCode, initChunk, writeChunk, addConstant};
use self::debug::disassembleChunk;
use self::vm::{/*initVM,*/ interpret};

/*
fn main() {
    loop {
        let mut line = String::new();
        match std::io::stdin().read_line(&mut line) {
            Ok(0) => {
                break;
            }
            Ok(n) => {
                print!("{}: {}", n, line);
            }
            Err(error) => {
                println!("err: {}", error);
                process::exit(1);
            }
        }
    }
}
 */

fn main() {
    let mut chunk = initChunk();
    let constant = addConstant(&mut chunk, 1.2);
    writeChunk(&mut chunk, OpCode::CONSTANT as u8, 123);
    writeChunk(&mut chunk, constant as u8, 123);

    let constant = addConstant(&mut chunk, 3.4);
    writeChunk(&mut chunk, OpCode::CONSTANT as u8, 123);
    writeChunk(&mut chunk, constant as u8, 123);

    writeChunk(&mut chunk, OpCode::ADD as u8, 123);

    let constant = addConstant(&mut chunk, 5.6);
    writeChunk(&mut chunk, OpCode::CONSTANT as u8, 123);
    writeChunk(&mut chunk, constant as u8, 123);

    writeChunk(&mut chunk, OpCode::DIVIDE as u8, 123);
    writeChunk(&mut chunk, OpCode::NEGATE as u8, 123);

    writeChunk(&mut chunk, OpCode::RETURN as u8, 123);
    disassembleChunk(&mut chunk, "test chunk");
    interpret(&chunk);
}
