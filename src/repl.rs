use std::io::*;
use std::process::*;

use crate::lexer::Lexer;
use crate::token::Token;

pub fn start(stdin: Stdin, stdout: Stdout) {
    loop {
        let mut out = stdout.lock();
        out.write_all(b">> ").unwrap();
        out.flush().unwrap();

        let mut line = String::new();
        stdin.read_line(&mut line).expect("Failed to read line...");

        let mut lexer = Lexer::new(&line);
        loop {
            let token = lexer.next_token();
            if token == Token::Ident("exit".to_string()) {
                println!("See you!!");
                exit(0);
            }
            match token {
                Token::Eof => break,
                _ => out.write_all(format!("{:?}\n", token).as_bytes()).unwrap(),
            }
        }
        out.flush().unwrap();
    }
}
