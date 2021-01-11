use crate::lexer::Lexer;
use crate::token::Token;

pub struct Parser<'a> {
    lexer: Lexer<'a>,
    cur_token: Token,
    next_token: Token,
}

impl<'a> Parser<'a> {
    pub fn new(lexer: Lexer<'a>) -> Self {
        let mut parser = Parser {
            lexer,
            cur_token: Token::Illegal,
            next_token: Token::Illegal,
        };
        // Call twice to set tokens to both `cur_token` and `next_token`.
        parser.next_token();
        parser.next_token();
        parser
    }

    pub fn parse_program(&self) {}

    fn next_token(&mut self) {
        self.cur_token = self.next_token.clone();
        self.next_token = self.lexer.next_token();
    }
}

#[cfg(test)]
mod tests {
    use crate::lexer::Lexer;
    use crate::parser::Parser;

    #[test]
    #[ignore]
    fn test_let_statements() {
        let input = r#"
            let x = 5;
            let y = 10;
            let foobar = 838383;
        "#;
        let lexer = Lexer::new(&input);
        let parser = Parser::new(lexer);
        let program = parser.parse_program().unwrap();
        assert_eq!(program.statements.len(), 3);

        let expected_identifers = vec!["x", "y", "foobar"];

        for (i, ident) in expected_identifers.iter().enumerate() {
            if let statement = program.statements[i] {
                assert_eq!(statement, ident);
            }
        }
    }
}
