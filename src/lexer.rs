use crate::token::{Token, TokenType};

pub struct Lexer {
    input: String,
    pos: usize,
    next_pos: usize,
    current_char: char,
}

impl Lexer {
    pub fn new(input: String) -> Lexer {
        let mut lexer = Lexer {
            input,
            pos: 0,
            next_pos: 0,
            current_char: 0 as char,
        };
        lexer.read_char();
        lexer
    }

    pub fn next_token(&mut self) -> Token {
        let token = match self.current_char {
            '=' => Token::new(TokenType::Assign, self.current_char),
            ';' => Token::new(TokenType::Semicolon, self.current_char),
            '(' => Token::new(TokenType::Lparen, self.current_char),
            ')' => Token::new(TokenType::Rparen, self.current_char),
            ',' => Token::new(TokenType::Comma, self.current_char),
            '+' => Token::new(TokenType::Plus, self.current_char),
            '{' => Token::new(TokenType::Lbrace, self.current_char),
            '}' => Token::new(TokenType::Rbrace, self.current_char),
            _ => Token::new(TokenType::Eof, self.current_char),
        };
        self.read_char();
        token
    }

    fn read_char(&mut self) {
        if self.next_pos >= self.input.len() {
            self.current_char = 0 as char;
        } else {
            self.current_char = self.input.chars().collect::<Vec<char>>()[self.next_pos];
        }
        self.pos = self.next_pos;
        self.next_pos += 1;
    }
}

#[cfg(test)]
extern crate speculate;

#[cfg(test)]
use speculate::speculate;

#[cfg(test)]
speculate! {
    use crate::lexer::{Lexer};
    use crate::token::{Token, TokenType};

    describe "Lexer#next_token" {
        before {
            let input = "=+(){},;".to_string();
            let expected = vec![
                Token::new(TokenType::Assign, '='),
                Token::new(TokenType::Plus, '+'),
                Token::new(TokenType::Lparen, '('),
                Token::new(TokenType::Rparen, ')'),
                Token::new(TokenType::Lbrace, '{'),
                Token::new(TokenType::Rbrace, '}'),
                Token::new(TokenType::Comma, ','),
                Token::new(TokenType::Semicolon, ';'),
                Token::new(TokenType::Eof, '\u{0}'),
            ];
            let mut lexer = Lexer::new(input);
        }

        it "should return the correct tokens from the passed literals" {
            for token in expected.iter() {
                assert_eq!(lexer.next_token(), *token);
            }
        }
    }
}
