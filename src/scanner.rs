use crate::token::Token;
use crate::token_type::TokenType::{self, *};
use crate::token_type::Literal;
use crate::error;

use std::collections::HashMap;

pub struct Scanner {
    source: String, 
    tokens: Vec<Token>,
    start: usize, 
    current: usize, 
    line: usize,
    keywords: HashMap<&'static str, TokenType>
}

impl Scanner {
    pub fn new(source: String) -> Self{
        Self {
            source, 
            tokens: Vec::new(), 
            start: 0, 
            current: 0, 
            line: 1,
            keywords: HashMap::from([
                ("and", AND),
                ("class", CLASS),
                ("else", ELSE),
                ("false", FALSE),
                ("for", FOR), 
                ("fun", FUN),
                ("if", IF), 
                ("nil", NIL),
                ("or", OR),
                ("print", PRINT),
                ("return", RETURN),
                ("super", SUPER),
                ("this", THIS),
                ("true", TRUE), 
                ("var", VAR),
                ("while", WHILE)
            ])
        }
    }

    pub fn scan_tokens(&mut self) -> Vec<Token> {
        while !self.is_at_end() {
            self.start = self.current; 
            self.scan_token();
        }

        self.tokens.push(Token::new(EOF, "".to_string(),None, self.line));
        return self.tokens.clone();

    }

    fn is_at_end(&self) -> bool {
        return self.current >= self.source.len();
    }

    fn scan_token(&mut self) {
        let c: char = self.advance();
        match c {
            '(' => self.add_token(LEFT_PAREN),
            ')' => self.add_token(RIGHT_PAREN),
            '{' => self.add_token(LEFT_BRACE),
            '}' => self.add_token(RIGHT_BRACE),
            ',' => self.add_token(COMMA),
            '.' => self.add_token(DOT),
            '-' => self.add_token(MINUS),
            '+' => self.add_token(PLUS),
            ';' => self.add_token(SEMICOLON),
            '*' => self.add_token(STAR),
            '!' => {let found = self.find('='); self.add_token(if found {BANG_EQUAL} else {BANG})},
            '=' => {let found = self.find('='); self.add_token(if found {EQUAL_EQUAL} else {EQUAL})},
            '>' => {let found = self.find('='); self.add_token(if found {GREATER_EQUAL} else {GREATER})},
            '<' => {let found = self.find('='); self.add_token(if found {LESS_EQUAL} else {LESS})},
            '/' => {
                if self.find('/') {
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                } else {
                    self.add_token(SLASH);
                }
            },
            ' ' => return,
            '\r' => return,
            '\t' => return, 
            '\n' => { self.line += 1; return},
            '"' => self.string(),


            _ => {
                if self.is_digit(c) { self.number() } else if self.is_alpha(c) {self.identifier();} else {error(self.line, "Unexpected character.")}
            }
        }
    }

    fn advance(&mut self) -> char {
        let c = self.source.chars().nth(self.current).unwrap();
        self.current += 1;
        c
    }

    fn add_token(&mut self, token: TokenType) {
        self.add_literal(token, None)
    }

    fn add_literal(&mut self, token: TokenType, literal: Option<Literal>) {
        let text = self.source[self.start..self.current].to_string();
        self.tokens.push(Token::new(token, text, literal, self.line))
    }

    fn find(&mut self, expected: char) -> bool {
        if self.is_at_end() || self.source.chars().nth(self.current) != Some(expected) {
            false
        } else {
            self.current += 1;
            true
        } 
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            '\0'
        } else {
            self.source.chars().nth(self.current).unwrap()
        }
    }

    fn string(&mut self) {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }

        if self.is_at_end() {
            error(self.line, "Unterminated string");
            return;
        }

        self.advance(); // closing "


        let text = &self.source[self.start + 1..self.current - 1];
        self.add_literal(STRING, Some(Literal::STRING(text.to_string())))
    }

    fn is_digit(&self, c: char) -> bool {
        return c >= '0' && c <= '9'
    }

    fn number(&mut self) {
        while self.is_digit(self.peek()) {
            self.advance();
        }

        if self.peek() == '.' && self.is_digit(self.peek_next()) {
            self.advance();
        }


        while self.is_digit(self.peek()) {
            self.advance();
        }

        let text = &self.source[self.start..self.current];
        self.add_literal(NUMBER, Some(Literal::NUMBER(text.parse().unwrap())));
    }

    fn peek_next(&self) -> char {
        if self.current + 1 >= self.source.len() {
            '\0'
        } else {
            self.source.chars().nth(self.current + 1).unwrap()
        }
    }

    fn identifier(&mut self) {
        while self.is_alphanumeric(self.peek()) {
            self.advance();
        }

        let text = &self.source[self.start..self.current];
        let typ = match self.keywords.get(&text) { None => IDENTIFIER, Some(t) => t.clone()};
        self.add_token(typ)
    }

    fn is_alpha(&self, c: char) -> bool {
        (c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z') || c == '_'
    }

    fn is_alphanumeric(&self, c: char) -> bool {
        self.is_alpha(c) || self.is_digit(c)
    }
}