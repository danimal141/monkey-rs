use crate::token::{Token, TokenType};

#[derive(Default)]
pub struct Lexer {
    input: String,
    pos: usize,
    next_pos: usize,
    ch: char,
}

impl Lexer {
    pub fn new(input: String) -> Lexer {
        let mut lexer = Lexer {
            input,
            pos: 0,
            next_pos: 0,
            ..Default::default()
        };
        lexer.read_char();
        lexer
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();
        let token = match self.ch {
            '=' => Token::new(TokenType::Assign, self.ch),
            '+' => Token::new(TokenType::Plus, self.ch),
            '-' => Token::new(TokenType::Minus, self.ch),
            '!' => Token::new(TokenType::Bang, self.ch),
            '/' => Token::new(TokenType::Slash, self.ch),
            '*' => Token::new(TokenType::Asterisk, self.ch),
            '<' => Token::new(TokenType::Lt, self.ch),
            '>' => Token::new(TokenType::Gt, self.ch),
            ';' => Token::new(TokenType::Semicolon, self.ch),
            '(' => Token::new(TokenType::Lparen, self.ch),
            ')' => Token::new(TokenType::Rparen, self.ch),
            ',' => Token::new(TokenType::Comma, self.ch),
            '{' => Token::new(TokenType::Lbrace, self.ch),
            '}' => Token::new(TokenType::Rbrace, self.ch),
            '\0' => Token::new(TokenType::Eof, self.ch),
            _ => {
                if self.is_letter() {
                    let literal = self.read_identifier();
                    return Token::new_with_literal(literal);
                } else if self.is_digit() {
                    let literal = self.read_number();
                    return Token {
                        token_type: TokenType::Int,
                        literal,
                    };
                } else {
                    Token::new(TokenType::Illegal, self.ch)
                }
            }
        };
        self.read_char();
        token
    }

    fn read_char(&mut self) {
        if self.next_pos >= self.input.len() {
            self.ch = '\0';
        } else {
            self.ch = self.input.chars().collect::<Vec<char>>()[self.next_pos];
        }
        self.pos = self.next_pos;
        self.next_pos += 1;
    }

    fn read_identifier(&mut self) -> String {
        let mut str = String::new();
        while self.is_letter() {
            str.push(self.ch);
            self.read_char();
        }
        str
    }

    fn read_number(&mut self) -> String {
        let mut str = String::new();
        while self.is_digit() {
            str.push(self.ch);
            self.read_char();
        }
        str
    }

    fn skip_whitespace(&mut self) {
        while self.ch.is_whitespace() {
            self.read_char();
        }
    }

    fn is_letter(&self) -> bool {
        self.ch.is_alphabetic() || self.ch == '_'
    }

    fn is_digit(&self) -> bool {
        self.ch.is_digit(10)
    }
}

#[cfg(test)]
extern crate speculate;

#[cfg(test)]
use speculate::speculate;

#[cfg(test)]
speculate! {
    use crate::lexer::{Lexer};

    describe "Lexer#next_token" {
        before {
            struct ExpectedToken {
                token_type: TokenType,
                literal: String,
            }

            let input = r#"
                let five = 5;
                let ten = 10;
                let add = fn(x, y) {
                    x + y;
                };
                let result = add(five, ten);

                !-/*5;
                5 < 10 > 5;

                if (5 < 10) {
                    return true;
                } else {
                    return false;
                }
            "#.to_string();
            let expected_tokens = [
                ExpectedToken { token_type: TokenType::Let, literal: "let".to_string() },
                ExpectedToken { token_type: TokenType::Ident, literal: "five".to_string() },
                ExpectedToken { token_type: TokenType::Assign, literal: "=".to_string() },
                ExpectedToken { token_type: TokenType::Int, literal: "5".to_string() },
                ExpectedToken { token_type: TokenType::Semicolon, literal: ";".to_string() },

                ExpectedToken { token_type: TokenType::Let, literal: "let".to_string() },
                ExpectedToken { token_type: TokenType::Ident, literal: "ten".to_string() },
                ExpectedToken { token_type: TokenType::Assign, literal: "=".to_string() },
                ExpectedToken { token_type: TokenType::Int, literal: "10".to_string() },
                ExpectedToken { token_type: TokenType::Semicolon, literal: ";".to_string() },

                ExpectedToken { token_type: TokenType::Let, literal: "let".to_string() },
                ExpectedToken { token_type: TokenType::Ident, literal: "add".to_string() },
                ExpectedToken { token_type: TokenType::Assign, literal: "=".to_string() },
                ExpectedToken { token_type: TokenType::Function, literal: "fn".to_string() },
                ExpectedToken { token_type: TokenType::Lparen, literal: "(".to_string() },
                ExpectedToken { token_type: TokenType::Ident, literal: "x".to_string() },
                ExpectedToken { token_type: TokenType::Comma, literal: ",".to_string() },
                ExpectedToken { token_type: TokenType::Ident, literal: "y".to_string() },
                ExpectedToken { token_type: TokenType::Rparen, literal: ")".to_string() },
                ExpectedToken { token_type: TokenType::Lbrace, literal: "{".to_string() },
                ExpectedToken { token_type: TokenType::Ident, literal: "x".to_string() },
                ExpectedToken { token_type: TokenType::Plus, literal: "+".to_string() },
                ExpectedToken { token_type: TokenType::Ident, literal: "y".to_string() },
                ExpectedToken { token_type: TokenType::Semicolon, literal: ";".to_string() },
                ExpectedToken { token_type: TokenType::Rbrace, literal: "}".to_string() },
                ExpectedToken { token_type: TokenType::Semicolon, literal: ";".to_string() },

                ExpectedToken { token_type: TokenType::Let, literal: "let".to_string() },
                ExpectedToken { token_type: TokenType::Ident, literal: "result".to_string() },
                ExpectedToken { token_type: TokenType::Assign, literal: "=".to_string() },
                ExpectedToken { token_type: TokenType::Ident, literal: "add".to_string() },
                ExpectedToken { token_type: TokenType::Lparen, literal: "(".to_string() },
                ExpectedToken { token_type: TokenType::Ident, literal: "five".to_string() },
                ExpectedToken { token_type: TokenType::Comma, literal: ",".to_string() },
                ExpectedToken { token_type: TokenType::Ident, literal: "ten".to_string() },
                ExpectedToken { token_type: TokenType::Rparen, literal: ")".to_string() },
                ExpectedToken { token_type: TokenType::Semicolon, literal: ";".to_string() },

                ExpectedToken { token_type: TokenType::Bang, literal: "!".to_string() },
                ExpectedToken { token_type: TokenType::Minus, literal: "-".to_string() },
                ExpectedToken { token_type: TokenType::Slash, literal: "/".to_string() },
                ExpectedToken { token_type: TokenType::Asterisk, literal: "*".to_string() },
                ExpectedToken { token_type: TokenType::Int, literal: "5".to_string() },
                ExpectedToken { token_type: TokenType::Semicolon, literal: ";".to_string() },

                ExpectedToken { token_type: TokenType::Int, literal: "5".to_string() },
                ExpectedToken { token_type: TokenType::Lt, literal: "<".to_string() },
                ExpectedToken { token_type: TokenType::Int, literal: "10".to_string() },
                ExpectedToken { token_type: TokenType::Gt, literal: ">".to_string() },
                ExpectedToken { token_type: TokenType::Int, literal: "5".to_string() },
                ExpectedToken { token_type: TokenType::Semicolon, literal: ";".to_string() },

                ExpectedToken { token_type: TokenType::If, literal: "if".to_string() },
                ExpectedToken { token_type: TokenType::Lparen, literal: "(".to_string() },
                ExpectedToken { token_type: TokenType::Int, literal: "5".to_string() },
                ExpectedToken { token_type: TokenType::Lt, literal: "<".to_string() },
                ExpectedToken { token_type: TokenType::Int, literal: "10".to_string() },
                ExpectedToken { token_type: TokenType::Rparen, literal: ")".to_string() },
                ExpectedToken { token_type: TokenType::Lbrace, literal: "{".to_string() },
                ExpectedToken { token_type: TokenType::Return, literal: "return".to_string() },
                ExpectedToken { token_type: TokenType::True, literal: "true".to_string() },
                ExpectedToken { token_type: TokenType::Semicolon, literal: ";".to_string() },
                ExpectedToken { token_type: TokenType::Rbrace, literal: "}".to_string() },
                ExpectedToken { token_type: TokenType::Else, literal: "else".to_string() },
                ExpectedToken { token_type: TokenType::Lbrace, literal: "{".to_string() },
                ExpectedToken { token_type: TokenType::Return, literal: "return".to_string() },
                ExpectedToken { token_type: TokenType::False, literal: "false".to_string() },
                ExpectedToken { token_type: TokenType::Semicolon, literal: ";".to_string() },
                ExpectedToken { token_type: TokenType::Rbrace, literal: "}".to_string() },

                ExpectedToken { token_type: TokenType::Eof, literal: "\0".to_string() },
            ];
            let mut lexer = Lexer::new(input);
        }

        it "should return the correct tokens from the passed literals" {
            for expected_token in expected_tokens.iter() {
                let current = lexer.next_token();
                assert_eq!(current.token_type, expected_token.token_type);
                assert_eq!(current.literal, expected_token.literal);
            }
        }
    }
}
