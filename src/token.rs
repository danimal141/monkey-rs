#[derive(Debug, PartialEq)]
pub enum TokenType {
    Illegal,
    Eof,

    // Identifiers and literals
    Ident, // add, foobar, x, y, ...
    Int,   // 12345

    // Operators
    Assign,
    Plus,

    // Delimiters
    Comma,
    Semicolon,
    Lparen,
    Rparen,
    Lbrace,
    Rbrace,

    // Keywords
    Function,
    Let,
}

#[derive(Debug, PartialEq)]
pub struct Token {
    pub token_type: TokenType,
    pub literal: String,
}

impl Token {
    pub fn new(token_type: TokenType, ch: char) -> Token {
        let mut str = String::new();
        str.push(ch);

        Token {
            token_type,
            literal: str,
        }
    }

    pub fn new_with_literal(literal: String) -> Token {
        let token_type = match literal.as_str() {
            "let" => TokenType::Let,
            "fn" => TokenType::Function,
            _ => TokenType::Ident,
        };

        Token {
            token_type,
            literal,
        }
    }
}
