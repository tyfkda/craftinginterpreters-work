#![feature(ptr_offset_from)]

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

use self::vm::{interpret, InterpretError};

fn main() -> Result<(), String> {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        repl().map_err(|e| format!("{:?}", e))?;
    } else if args.len() == 2 {
        runFile(&args[1]).map_err(|e| format!("{:?}", e))?;
    } else {
        panic!("Usage: rlox [path]");
    }

    Ok(())
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
        if let Err(_e) = interpret(&line) {
            // Ignore.
        }
    }
}

fn runFile(path: &str) -> Result<(), InterpretError> {
    let source = readFile(path).map_err(|_| InterpretError::COMPILE_ERROR)?;
    interpret(&source)
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
