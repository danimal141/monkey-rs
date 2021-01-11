#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Token {
    Illegal,
    Eof,

    // Identifiers and literals
    Ident(String), // add, foobar, x, y, ...
    Int(i32),      // 12345

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

impl Token {
    pub fn lookup_token_type(ident: &str) -> Token {
        match ident {
            "let" => Token::Let,
            "fn" => Token::Function,
            "true" => Token::True,
            "false" => Token::False,
            "if" => Token::If,
            "else" => Token::Else,
            "return" => Token::Return,
            _ => Token::Ident(ident.to_string()),
        }
    }
}
