use crate::token::Token;
use crate::token_type::TokenType;
use strum_macros::Display;
use std::fmt;

#[derive(Debug, Clone)]
pub enum Expression {
    Binary(BinaryExpression),
    Grouping(GroupingExpression),
    Literal(LiteralExpression),
    Unary(UnaryExpression)
}


#[derive(Debug, Clone)]
pub struct BinaryExpression {
    pub left: Box<Expression>,
    pub operator: Token,
    pub right: Box<Expression>
}

#[derive(Debug, Clone)]
pub struct GroupingExpression {
    pub expression: Box<Expression>
}

#[derive(Debug, Clone, Display)]
pub enum LiteralExpression {
    Boolean(bool),
    Null(Token),
    String(Token),
    Number(Token),
}

#[derive(Debug, Clone)]
pub struct UnaryExpression {
    pub operator: Token, 
    pub right: Box<Expression>
}

impl Expression {
    pub fn binary(left: Expression, operator: Token, right: Expression) -> Self {
        Self::Binary(BinaryExpression {
            left: Box::new(left),
            operator, 
            right: Box::new(right)
        }
        )
    }

    pub fn grouping(expression: Expression) -> Self {
        Self::Grouping(GroupingExpression{
            expression: Box::new(expression)
        })
    }

    pub fn unary(operator: Token, right: Expression) -> Self {
        Self::Unary(UnaryExpression{
            operator,
            right: Box::new(right)
        })
    }

    pub fn literal(token: Token) -> Self {
        match token.token_type {
            TokenType::TRUE => Self::Literal(LiteralExpression::Boolean(true)),
            TokenType::FALSE => Self::Literal(LiteralExpression::Boolean(false)),
            TokenType::NIL => Self::Literal(LiteralExpression::Null(token)),
            TokenType::STRING => Self::Literal(LiteralExpression::String(token)),
            TokenType::NUMBER => Self::Literal(LiteralExpression::Number(token)),
            _ => Self::Literal(LiteralExpression::Null(token)) // base case but should refine
        }
    }

    pub fn parenthesize(&self, name: String, expressions: Vec<Expression>) -> String {
        let mut s = String::new();

        s.push('(');
        s.push_str(&name);
        for expression in expressions {
            s.push(' ');
            let part = match expression {
                        Expression::Binary(e) => self.parenthesize(e.operator.lexeme.clone(), vec![*e.left.clone(), *e.right.clone()]),
                        Expression::Grouping(e) => self.parenthesize("group".to_string(), vec![*e.expression.clone()]),
                        Expression::Literal(e) => { 
                            match e { 
                                LiteralExpression::Null(_) => "nil".to_string(), 
                                LiteralExpression::Boolean(b) => b.to_string(), 
                                LiteralExpression::Number(t) => t.to_string(), 
                                LiteralExpression::String(t) => t.to_string()
                            }
                        },
                        Expression::Unary(e) => self.parenthesize(e.operator.lexeme.clone(), vec![*e.right.clone()])

                };
            s.push_str(&part);
        }
        s.push(')');
        s
    }

    pub fn print(&self) -> String {
        match self {
            Expression::Binary(e) => self.parenthesize(e.operator.lexeme.clone(), vec![*e.left.clone(), *e.right.clone()]),
            Expression::Grouping(e) => self.parenthesize("group".to_string(), vec![*e.expression.clone()]),
            Expression::Literal(e) => { 
                match e { 
                    LiteralExpression::Null(_) => "nil".to_string(), 
                    LiteralExpression::Boolean(b) => b.to_string(), 
                    LiteralExpression::Number(t) => t.to_string(), 
                    LiteralExpression::String(t) => t.to_string()
                }
            },
            Expression::Unary(e) => self.parenthesize(e.operator.lexeme.clone(), vec![*e.right.clone()])
        }
    }
}

impl fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.print())
    }
}