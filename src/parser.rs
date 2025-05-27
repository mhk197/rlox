use crate::token::Token;
use crate::token_error;
use crate::token_type::TokenType;
use crate::token_type::TokenType::*;
use crate::ast::{Expression, Statement, VarDeclaration};

struct ParseError;

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
    pub had_error: bool
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            current: 0,
            had_error: false
        }
    }

    fn expression(&mut self) -> Result<Expression, ParseError> {
        self.equality()
    }

    fn equality(&mut self) -> Result<Expression, ParseError>  {
        let mut expression = self.comparison()?;
        while self.match_(vec![BANG_EQUAL, EQUAL_EQUAL]) {
            let operator = self.previous();
            let right = self.comparison()?;

            expression = Expression::binary(expression.clone(), operator.clone(), right.clone());
        }

        Ok(expression)
    }

    fn match_(&mut self, types_: Vec<TokenType>) -> bool {
        for t in types_ {
            if self.check(t) {
                self.advance();
                return true
            }
        }
        false
    }

    fn check(&self, token_type: TokenType) -> bool {
        if self.is_at_end() {
            return false 
        }

        self.peek().token_type == token_type
    }

    fn advance(&mut self) -> Token {
        if !self.is_at_end() {
            self.current += 1;
        }

        self.previous() 
    }

    fn is_at_end(&self) -> bool {
        self.peek().token_type == EOF
    }

    fn peek(&self) -> Token {
        self.tokens[self.current].clone()
    }

    fn previous(&self) -> Token {
        self.tokens[self.current - 1].clone()
    }

    fn comparison(&mut self) -> Result<Expression, ParseError>  {
        let mut expression = self.term()?;
        while self.match_(vec![LESS, LESS_EQUAL, GREATER, GREATER_EQUAL]) {
            let operator = self.previous();
            let right = self.term()?;

            expression = Expression::binary(expression.clone(), operator.clone(), right.clone());
        }

        Ok(expression)
    }

    fn term(&mut self) -> Result<Expression, ParseError> {
        let mut expression = self.factor()?;
        while self.match_(vec![MINUS, PLUS]) {
            let operator = self.previous();
            let right = self.factor()?;

            expression = Expression::binary(expression.clone(), operator.clone(), right.clone());
        }

        Ok(expression)
    }

    fn factor(&mut self) -> Result<Expression, ParseError>  {
        let mut expression = self.unary()?;
        while self.match_(vec![SLASH, STAR]) {
            let operator = self.previous();
            let right = self.unary()?;

            expression = Expression::binary(expression.clone(), operator.clone(), right.clone());
        }

        Ok(expression)
    }

    fn unary(&mut self) -> Result<Expression, ParseError>  {
        if self.match_(vec![MINUS, BANG]) {
            let operator = self.previous();
            let right = self.primary()?;

            return Ok(Expression::unary(operator.clone(), right.clone()))
        }

        self.primary()
    }

    fn primary(&mut self) -> Result<Expression, ParseError> {
        if self.match_(vec![FALSE, TRUE, NIL, STRING, NUMBER]) {
            return Ok(Expression::literal(self.previous()))
        } else if self.match_(vec![LEFT_PAREN]) { // must be parentheses
            let expression = self.expression()?;
            self.consume(RIGHT_PAREN, "Expect ')' after expression.")?;
            return Ok(Expression::grouping(expression)) 
        } else if self.match_(vec![TokenType::IDENTIFIER]){
            return Ok(Expression::variable(self.previous()))
        } else {
            Err(self.parse_error(self.peek(), "Expect expression."))
        }
    }

    fn consume(&mut self, type_: TokenType, message: &'static str) -> Result<Token, ParseError> {
        if self.check(type_) {
            Ok(self.advance())
        } else {
            Err(self.parse_error(self.peek(), message))
        }
    }

    fn parse_error(&self, token: Token, message: &'static str) -> ParseError {
        token_error(token, message);
        ParseError{}
    }

    fn synchronize(&mut self) {
        self.advance();

        while !self.is_at_end() {
            if self.previous().token_type == SEMICOLON {
                return
            }

            match self.peek().token_type {
                TokenType::CLASS => return,
                TokenType::FUN => return,
                TokenType::VAR => return,
                TokenType::FOR => return,
                TokenType::IF => return,
                TokenType::WHILE => return,
                TokenType::PRINT => return,
                TokenType::RETURN => return,
                _ => ()
            }

            self.advance();
        }
    }

    fn print_statement(&mut self) -> Result<Statement, ParseError> {
        let value = self.expression()?;
        self.consume(TokenType::SEMICOLON, "Expect ';' after value.");
        Ok(Statement::Print(value))
    }

    fn expression_statement(&mut self) -> Result<Statement, ParseError> {
        let value = self.expression()?;
        self.consume(TokenType::SEMICOLON, "Expect ';' after value.");
        Ok(Statement::Expression(value))
    }

    fn statement(&mut self) -> Result<Statement, ParseError> {
        if self.match_(vec![TokenType::PRINT]) {
            return self.print_statement()
        }

        self.expression_statement()
    }

    fn var_declaration(&mut self) -> Result<Statement, ParseError> {
        let name = self.consume(TokenType::IDENTIFIER, "Expect variable name.")?;

        let mut initializer: Option<Expression> = None;
        if self.match_(vec![TokenType::EQUAL]) {
            initializer = Some(self.expression()?);
        }

        self.consume(TokenType::SEMICOLON, "Expect ';' after variable declaration.");
        return Ok(Statement::VarDeclaration(VarDeclaration{name, initializer}))
    }

    fn declaration(&mut self) -> Result<Statement, ParseError> {
        if self.match_(vec![TokenType::VAR]) {
            return self.var_declaration()
        } else {
            return self.statement()
        }
    }

    pub fn parse(&mut self) -> Vec<Option<Statement>> {
        let mut statements: Vec<Option<Statement>> = Vec::new();
        while !self.is_at_end() {
            match self.declaration() {
                Ok(s) => statements.push(Some(s)),
                Err(_) => {statements.push(None); self.had_error = true; self.synchronize();}
            }
        }
        statements
    }
}