use crate::token::Token;

#[derive(Default)]
pub struct Lexer<'a> {
    input: &'a str,
    pos: usize,
    next_pos: usize,
    ch: char,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Lexer {
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
            '=' => {
                if self.peek_char() == '=' {
                    self.read_char();
                    Token::Eq
                } else {
                    Token::Assign
                }
            }
            '!' => {
                if self.peek_char() == '=' {
                    self.read_char();
                    Token::NotEq
                } else {
                    Token::Bang
                }
            }
            '+' => Token::Plus,
            '-' => Token::Minus,
            '/' => Token::Slash,
            '*' => Token::Asterisk,
            '<' => Token::Lt,
            '>' => Token::Gt,
            ';' => Token::Semicolon,
            '(' => Token::Lparen,
            ')' => Token::Rparen,
            ',' => Token::Comma,
            '{' => Token::Lbrace,
            '}' => Token::Rbrace,
            '\0' => Token::Eof,
            _ => {
                if self.is_letter() {
                    let ident = self.read_identifier();
                    return Token::lookup_token_type(&ident);
                } else if self.is_digit() {
                    let number = self.read_number();
                    return Token::Int(number);
                } else {
                    Token::Illegal
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

    fn peek_char(&self) -> char {
        if self.next_pos >= self.input.len() {
            return '\0';
        }
        self.input.chars().collect::<Vec<char>>()[self.next_pos]
    }

    fn read_identifier(&mut self) -> String {
        let mut str = String::new();
        while self.is_letter() {
            str.push(self.ch);
            self.read_char();
        }
        str
    }

    fn read_number(&mut self) -> i32 {
        let mut str = String::new();
        while self.is_digit() {
            str.push(self.ch);
            self.read_char();
        }
        // Convert String -> i32
        str.parse().unwrap()
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
mod tests {
    use crate::lexer::Lexer;
    use crate::token::Token;

    #[test]
    fn text_next_token() {
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

            10 == 10;
            10 != 9;
        "#;
        let expected_tokens = [
            Token::Let,
            Token::Ident("five".to_string()),
            Token::Assign,
            Token::Int(5),
            Token::Semicolon,
            Token::Let,
            Token::Ident("ten".to_string()),
            Token::Assign,
            Token::Int(10),
            Token::Semicolon,
            Token::Let,
            Token::Ident("add".to_string()),
            Token::Assign,
            Token::Function,
            Token::Lparen,
            Token::Ident("x".to_string()),
            Token::Comma,
            Token::Ident("y".to_string()),
            Token::Rparen,
            Token::Lbrace,
            Token::Ident("x".to_string()),
            Token::Plus,
            Token::Ident("y".to_string()),
            Token::Semicolon,
            Token::Rbrace,
            Token::Semicolon,
            Token::Let,
            Token::Ident("result".to_string()),
            Token::Assign,
            Token::Ident("add".to_string()),
            Token::Lparen,
            Token::Ident("five".to_string()),
            Token::Comma,
            Token::Ident("ten".to_string()),
            Token::Rparen,
            Token::Semicolon,
            Token::Bang,
            Token::Minus,
            Token::Slash,
            Token::Asterisk,
            Token::Int(5),
            Token::Semicolon,
            Token::Int(5),
            Token::Lt,
            Token::Int(10),
            Token::Gt,
            Token::Int(5),
            Token::Semicolon,
            Token::If,
            Token::Lparen,
            Token::Int(5),
            Token::Lt,
            Token::Int(10),
            Token::Rparen,
            Token::Lbrace,
            Token::Return,
            Token::True,
            Token::Semicolon,
            Token::Rbrace,
            Token::Else,
            Token::Lbrace,
            Token::Return,
            Token::False,
            Token::Semicolon,
            Token::Rbrace,
            Token::Int(10),
            Token::Eq,
            Token::Int(10),
            Token::Semicolon,
            Token::Int(10),
            Token::NotEq,
            Token::Int(9),
            Token::Semicolon,
            Token::Eof,
        ];
        let mut lexer = Lexer::new(input);

        for expected in expected_tokens.iter() {
            let actual = lexer.next_token();
            assert_eq!(&actual, expected);
        }
    }
}
