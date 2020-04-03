#![allow(non_camel_case_types, non_snake_case)]

use num_traits::FromPrimitive;
use std::mem::MaybeUninit;

use super::chunk::{initChunk, Chunk, OpCode};
use super::compiler::{compile};
use super::debug::{disassembleInstruction};
use super::value::*;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum InterpretError {
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

pub fn interpret<'a>(source: &'a str) -> Result<(), InterpretError> {
    let mut chunk = initChunk();
    if !compile(source, &mut chunk) {
        return Err(InterpretError::COMPILE_ERROR);
    }

    let stack: [MaybeUninit<Value>; STACK_MAX] = unsafe { MaybeUninit::uninit().assume_init() };
    let stack: [Value; STACK_MAX] = unsafe { std::mem::transmute::<_, [Value; STACK_MAX]>(stack) };

    let mut vm = VM {
        chunk: &chunk,
        ip: &chunk.code,
        stack,
        stackTop: 0,
    };

    run(&mut vm)
}

fn resetStack(vm: &mut VM) {
    vm.stackTop = 0;
}

fn runtimeError(vm: &mut VM, message: &str) {
    eprintln!("{}", message);

    let instruction = unsafe { (&vm.ip[0] as *const u8).offset_from(&vm.chunk.code[0] as *const u8) - 1 } as usize;
    let line = vm.chunk.lines[instruction];
    eprintln!("[line {}] in script\n", line);

    resetStack(vm);
}

fn push(vm: &mut VM, value: Value) {
    vm.stack[vm.stackTop] = value;
    vm.stackTop += 1;
}

fn pop<'a>(vm: &'a mut VM) -> &'a Value {
    vm.stackTop -= 1;
    &vm.stack[vm.stackTop]
}

fn peek<'a>(vm: &'a VM, distance: usize) -> &'a Value {
    &vm.stack[vm.stackTop - distance - 1]
}

fn isFalsey(value: &Value) -> bool {
    AS_BOOL(value).map_or_else(|| IS_NIL(value), |b| !b)
}

fn binary_op(vm: &mut VM, op: fn(f64, f64) -> Value) -> Result<(), InterpretError> {
    if AS_NUMBER(peek(vm, 0)).is_none() || AS_NUMBER(peek(vm, 1)).is_none() {
        runtimeError(vm, "Operands must be numbers.");
        return Err(InterpretError::RUNTIME_ERROR);
    }

    let b = AS_NUMBER(pop(vm)).unwrap();
    let a = AS_NUMBER(pop(vm)).unwrap();
    push(vm, op(a, b));
    Ok(())
}

fn run(vm: &mut VM) -> Result<(), InterpretError> {
    loop {
        print!("          ");
        for i in 0..vm.stackTop {
            print!("[ ");
            printValue(&vm.stack[i]);
            print!(" ]");
        }
        print!("\n");
        disassembleInstruction(vm.chunk, (vm.ip.as_ptr() as usize) - ((&vm.chunk.code as &[u8]).as_ptr() as usize));

        let instruction = FromPrimitive::from_u8(READ_BYTE(vm)).unwrap();
        match instruction {
            OpCode::CONSTANT => {
                let constant = READ_CONSTANT(vm).clone();
                push(vm, constant);
            }
            OpCode::NIL => { push(vm, Value::NIL); }
            OpCode::TRUE => { push(vm, Value::BOOL(true)); }
            OpCode::FALSE => { push(vm, Value::BOOL(false)); }

            OpCode::EQUAL => {
                let b = peek(vm, 0);
                let a = peek(vm, 1);
                let result = valuesEqual(a, b);
                pop(vm);
                pop(vm);
                push(vm, Value::BOOL(result));
            }

            OpCode::GREATER => { binary_op(vm, |a, b| Value::BOOL(a > b))?; }
            OpCode::LESS => { binary_op(vm, |a, b| Value::BOOL(a < b))?; }

            OpCode::ADD => { binary_op(vm, |a, b| Value::NUMBER(a + b))?; }
            OpCode::SUBTRACT => { binary_op(vm, |a, b| Value::NUMBER(a - b))?; }
            OpCode::MULTIPLY => { binary_op(vm, |a, b| Value::NUMBER(a * b))?; }
            OpCode::DIVIDE => { binary_op(vm, |a, b| Value::NUMBER(a / b))?; }
            OpCode::NOT => {
                let b = isFalsey(pop(vm));
                push(vm, Value::BOOL(b));
            }
            OpCode::NEGATE => {
                if let Some(v) = AS_NUMBER(peek(vm, 0)) {
                    pop(vm);
                    push(vm, Value::NUMBER(-v));
                } else {
                    runtimeError(vm, "Operand must be a number.");
                    return Err(InterpretError::RUNTIME_ERROR);
                }
            }
            OpCode::RETURN => {
                printValue(&pop(vm));
                print!("\n");
                return Ok(())
            }
        }
    }
}

fn READ_BYTE(vm: &mut VM) -> u8 {
    let result = vm.ip[0];
    vm.ip = &vm.ip[1 .. vm.ip.len()];
    result
}

fn READ_CONSTANT<'a>(vm: &'a mut VM) -> &'a Value {
    let index = READ_BYTE(vm);
    &vm.chunk.constants.values[index as usize]
}
