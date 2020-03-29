#![allow(non_snake_case)]

use std::env;
use std::fs;
use std::io::{self, BufRead, Write};

mod chunk;
mod compiler;
mod debug;
mod scanner;
mod value;
mod vm;

use self::vm::{interpret, InterpretResult};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        repl();
    } else if args.len() == 2 {
        runFile(&args[1]);
    } else {
        panic!("Usage: rlox [path]");
    }
}

fn repl() -> Result<(), std::io::Error> {
    let stdin = io::stdin();
    let mut line = String::new();
    loop {
        print!("> ");
        io::stdout().flush()?;
        line.clear();
        stdin.lock().read_line(&mut line)?;

        line.push_str("\0");
        interpret(&line);
    }
}

fn runFile(path: &str) -> Result<(), InterpretResult> {
    let source = readFile(path).map_err(|_| InterpretResult::COMPILE_ERROR)?;
    let result = interpret(&source);

    match result {
        InterpretResult::OK => { Ok(()) }
        InterpretResult::COMPILE_ERROR => { Err(result) }
        InterpretResult::RUNTIME_ERROR => { Err(result) }
    }
}

fn readFile(path: &str) -> Result<String, std::io::Error> {
    match fs::read(path) {
        Result::Ok(buffer) => {
            let mut s = String::from_utf8(buffer).unwrap();
            s.push_str("\0");
            Ok(s)
        },
        Result::Err(err) => {
            Err(err)
        }
    }
}
