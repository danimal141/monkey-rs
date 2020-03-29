use std::io::*;

use crate::lexer::Lexer;
use crate::token::TokenType;

pub fn start(stdin: Stdin, stdout: Stdout) {
    loop {
        let mut out = stdout.lock();
        out.write_all(b">> ").unwrap();
        out.flush().unwrap();

        let mut line = String::new();
        stdin.read_line(&mut line).expect("Failed to read line...");

        let mut lexer = Lexer::new(line);
        loop {
            let token = lexer.next_token();
            match token.token_type {
                TokenType::Eof => break,
                _ => {
                    out.write_all(format!("{:?}\n", token).as_bytes()).unwrap();
                }
            }
        }
        out.flush().unwrap();
    }
}
