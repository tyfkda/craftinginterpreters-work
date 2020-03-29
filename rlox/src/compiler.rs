use super::scanner::{initScanner, scanToken, TokenType};

pub fn compile<'a>(source: &'a str) {
    let mut scanner = initScanner(source);
    let mut line = -1;
    loop {
        let token = scanToken(&mut scanner);
        if token.line != line {
            print!("{:4} ", token.line);
            line = token.line;
        } else {
            print!("   | ");
        }
        println!("{:?} '{:}", token.token_type, token.start);

        if token.token_type == TokenType::EOF {
            break;
        }
    }
}
