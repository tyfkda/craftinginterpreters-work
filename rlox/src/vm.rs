#![allow(non_snake_case)]

use num_traits::FromPrimitive;

use super::chunk::{Chunk, OpCode};
use super::debug::{disassembleInstruction, printValue};
use super::value::Value;

pub enum InterpretResult {
    OK,
    COMPILE_ERROR,
    RUNTIME_ERROR,
}

struct VM<'a> {
    chunk: &'a Chunk,
    ip: &'a [u8],
}

pub fn interpret<'a>(chunk: &'a Chunk) -> InterpretResult {
    let mut vm = VM {
        chunk,
        ip: &chunk.code,
    };

    run(&mut vm)
}


fn run(vm: &mut VM) -> InterpretResult {
    loop {
        disassembleInstruction(vm.chunk, (vm.ip.as_ptr() as usize) - ((&vm.chunk.code as &[u8]).as_ptr() as usize));

        let instruction = FromPrimitive::from_u8(READ_BYTE(vm)).unwrap();
        match instruction {
            OpCode::CONSTANT => {
                let constant = READ_CONSTANT(vm);
                printValue(constant);
                println!("");
            }
            OpCode::RETURN => {
                return InterpretResult::OK;
            }
        }
    }
}

fn READ_BYTE(vm: &mut VM) -> u8 {
    let result = vm.ip[0];
    vm.ip = &vm.ip[1 .. vm.ip.len()];
    result
}

fn READ_CONSTANT(vm: &mut VM) -> Value {
    let index = READ_BYTE(vm);
    vm.chunk.constants.values[index as usize]
}
