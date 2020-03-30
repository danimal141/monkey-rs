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
    Minus,
    Bang,
    Asterisk,
    Slash,
    Lt,
    Gt,
    Eq,
    NotEq,

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
    True,
    False,
    If,
    Else,
    Return,
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

    pub fn from_identifier(ident: String) -> Token {
        let token_type = lookup_token_type(&ident);
        Token {
            token_type,
            literal: ident,
        }
    }
}

fn lookup_token_type(ident: &str) -> TokenType {
    match ident {
        "let" => TokenType::Let,
        "fn" => TokenType::Function,
        "true" => TokenType::True,
        "false" => TokenType::False,
        "if" => TokenType::If,
        "else" => TokenType::Else,
        "return" => TokenType::Return,
        _ => TokenType::Ident,
    }
}
