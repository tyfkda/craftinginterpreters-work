#![allow(non_snake_case)]

use num_traits::FromPrimitive;

use std::mem::MaybeUninit;

use super::chunk::{Chunk, OpCode};
use super::debug::{disassembleInstruction, printValue};
use super::value::Value;

pub enum InterpretResult {
    OK,
    COMPILE_ERROR,
    RUNTIME_ERROR,
}

const STACK_MAX: usize = 256;

struct VM<'a> {
    chunk: &'a Chunk,
    ip: &'a [u8],
    stack: [Value; STACK_MAX],
    stackTop: usize,
}

pub fn interpret<'a>(chunk: &'a Chunk) -> InterpretResult {
    let mut stack: [MaybeUninit<Value>; STACK_MAX] = unsafe { MaybeUninit::uninit().assume_init() };
    for (_i, slot) in stack.iter_mut().enumerate() {
        *slot = MaybeUninit::new(0.0);
    }
    let stack = unsafe { std::mem::transmute::<_, [Value; STACK_MAX]>(stack) };

    let mut vm = VM {
        chunk,
        ip: &chunk.code,
        stack,
        stackTop: 0,
    };

    run(&mut vm)
}

fn resetStack(vm: &mut VM) {
    vm.stackTop = 0;
}

fn push(vm: &mut VM, value: Value) {
    vm.stack[vm.stackTop] = value;
    vm.stackTop += 1;
}

fn pop(vm: &mut VM) -> Value {
    vm.stackTop -= 1;
    vm.stack[vm.stackTop]
}

fn run(vm: &mut VM) -> InterpretResult {
    loop {
        print!("          ");
        for i in 0..vm.stackTop {
            print!("[ ");
            printValue(vm.stack[i]);
            print!(" ]");
        }
        print!("\n");
        disassembleInstruction(vm.chunk, (vm.ip.as_ptr() as usize) - ((&vm.chunk.code as &[u8]).as_ptr() as usize));

        let instruction = FromPrimitive::from_u8(READ_BYTE(vm)).unwrap();
        match instruction {
            OpCode::CONSTANT => {
                let constant = READ_CONSTANT(vm);
                push(vm, constant);
            }
            OpCode::RETURN => {
                printValue(pop(vm));
                print!("\n");
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
