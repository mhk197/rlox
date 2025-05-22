use crate::token_type::{TokenType, Literal};

pub struct Token<'a> {
    token_type: TokenType,
    lexeme: &'a str,
    literal: Option<Literal>,
    line: usize
}

impl<'a> Token<'a> {
    pub fn new(token_type: TokenType, lexeme: &'a str, literal: Option<Literal>, line: usize) -> Self {
        Self {
            token_type,
            lexeme,
            literal,
            line
        }
    }

    pub fn to_string(&self) -> String {
        format!("{} {} {}", self.token_type, self.lexeme, self.literal.clone().unwrap_or(Literal::IDENTIFIER) )
    }
}