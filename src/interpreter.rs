use crate::{ast::{BinaryExpression, Expression, GroupingExpression, LiteralExpression, Statement, UnaryExpression, VarExpression}, environment::Environment};
use strum_macros::Display;
use crate::token_type::{Literal, TokenType};

#[derive(Display, Debug, Clone)]
pub enum Value {
    Boolean(bool),
    Null,
    String(String),
    Number(f32)
}

#[derive(Debug)]
pub struct InterpreterError;

impl Value {
    pub fn is_truthy(&self) -> bool {
        match self {
            Self::Null => false, 
            Self::Boolean(b) => *b, 
            _ => true
        }
    }

    pub fn negate(&self) -> Result<Self, InterpreterError> {
        match self {
            Self::Number(n) => Ok(Self::Number(-n)),
            _ => Err(InterpreterError)
        }
    }

    pub fn not(&self) -> Result<Self, InterpreterError> {
        match self.is_truthy() {
            false => Ok(Self::Boolean(true)), 
            true => Ok(Self::Boolean(false))
        }
    }

    pub fn add(&self, other: Value) -> Result<Self, InterpreterError>{
        match (self, other) {
            (Self::Number(n1), Self::Number(n2)) => Ok(Self::Number(n1 + n2)),
            (Self::String(s1), Self::String(s2)) => Ok(Self::String(format!("{s1}{s2}"))),
            (_, _) => Err(InterpreterError)
        }
    }

    pub fn subtract(&self, other: Value) -> Result<Self, InterpreterError>{
        match (self, other) {
            (Self::Number(n1), Self::Number(n2)) => Ok(Self::Number(n1 - n2)),
            (_, _) => Err(InterpreterError)
        }
    }

    pub fn divide(&self, other: Value) -> Result<Self, InterpreterError>{
        match (self, other) {
            (Self::Number(n1), Self::Number(n2)) => Ok(Self::Number(n1/n2)),
            (_, _) => Err(InterpreterError)
        }
    }

    pub fn multiply(&self, other: Value) -> Result<Self, InterpreterError>{
        match (self, other) {
            (Self::Number(n1), Self::Number(n2)) => Ok(Self::Number(n1 * n2)),
            (_, _) => Err(InterpreterError)
        }
    }

    pub fn greater(&self, other: Value) -> Result<Self, InterpreterError>{
        match (self, other) {
            (Self::Number(n1), Self::Number(n2)) => Ok(Self::Boolean(n1 > &n2)),
            (_, _) => Err(InterpreterError)
        }
    }

    pub fn greater_equal(&self, other: Value) -> Result<Self, InterpreterError>{
        match (self, other) {
            (Self::Number(n1), Self::Number(n2)) => Ok(Self::Boolean(n1 >= &n2)),
            (_, _) => Err(InterpreterError)
        }
    }

    pub fn less(&self, other: Value) -> Result<Self, InterpreterError>{
        match (self, other) {
            (Self::Number(n1), Self::Number(n2)) => Ok(Self::Boolean(n1 < &n2)),
            (_, _) => Err(InterpreterError)
        }
    }

    pub fn less_equal(&self, other: Value) -> Result<Self, InterpreterError>{
        match (self, other) {
            (Self::Number(n1), Self::Number(n2)) => Ok(Self::Boolean(n1 <= &n2)),
            (_, _) => Err(InterpreterError)
        }
    }

    pub fn is_equal (&self, other: Value) -> bool {
        match (self, other) {
            (Self::Number(n1), Self::Number(n2)) => (*n1) == n2,
            (Self::Boolean(b1), Self::Boolean(b2)) => (*b1) == b2,
            (Self::String(s1), Self::String(s2)) => (*s1) == s2,
            (Self::Null, Self::Null) => true,
            (_, _) => false
        }
    }

    pub fn equals (&self, other: Value) -> Result<Self, InterpreterError>{
        Ok(Self::Boolean(self.is_equal(other)))
    }

    pub fn not_equals (&self, other: Value) -> Result<Self, InterpreterError>{
        Ok(Self::Boolean(!self.is_equal(other)))
    }

    pub fn stringify(&self) -> String {
        match self {
            Self::Number(n) => {if n.fract() == 0.0 {(*n as i32).to_string()} else {n.to_string()}},
            Self::Boolean(b) => b.to_string(), 
            Self::String(s) => s.clone(),
            Self::Null => String::from("nil")
        }
    }
}

pub struct Interpreter {
    environment: Environment
}

impl Interpreter {
    pub fn new() -> Self {
        Self {
            environment: Environment::new()
        }
    }

    fn binary(&self, expression: BinaryExpression) -> Result<Value, InterpreterError>{
        let left = self.evaluate(*expression.left)?;
        let right = self.evaluate(*expression.right)?;

        match expression.operator.token_type {
            TokenType::PLUS => left.add(right),
            TokenType::MINUS => left.subtract(right),
            TokenType::SLASH => left.divide(right),
            TokenType::STAR => left.multiply(right),

            TokenType::GREATER => left.greater(right),
            TokenType::GREATER_EQUAL => left.greater_equal(right),
            TokenType::LESS => left.less(right),
            TokenType::LESS_EQUAL => left.less_equal(right),
            TokenType::BANG_EQUAL => left.not_equals(right),
            TokenType::EQUAL_EQUAL => left.equals(right),
            _ => Err(InterpreterError)
        }
    }

    fn grouping(&self, expression: GroupingExpression) -> Result<Value, InterpreterError> {
        self.evaluate(*expression.expression)
    }

    fn unary(&self, expression: UnaryExpression) -> Result<Value, InterpreterError> {
        let right = self.evaluate(*expression.right)?;

        match expression.operator.token_type {
            TokenType::MINUS => right.negate(),
            TokenType::BANG => right.not(),
            _ => Err(InterpreterError {})
        }
    }

    fn literal(&self, expression: LiteralExpression) -> Result<Value, InterpreterError> {
        match expression {
            LiteralExpression::Boolean(b) => Ok(Value::Boolean(b)),
            LiteralExpression::Null(_) => Ok(Value::Null),
            LiteralExpression::String(t) => {
                match t.literal {
                    Some(Literal::STRING(s)) => Ok(Value::String(s)),
                    _ => Err(InterpreterError {})
                }
            },
            LiteralExpression::Number(t) => {
                match t.literal {
                    Some(Literal::NUMBER(n)) => Ok(Value::Number(n)),
                    _ => Err(InterpreterError {})
                }
            }
        }
    }

    fn variable(&self, expression: VarExpression) -> Result<Value, InterpreterError> {
        self.environment.get(expression.name)
    }

    fn statement(&mut self, statement: Statement) -> Result<Value, InterpreterError> {
        match statement {
            Statement::Expression(e) => self.evaluate(e),
            Statement::Print(e) => {
                let val = self.evaluate(e)?; 
                println!("{}", val.stringify());
                Ok(val)
            },
            Statement::VarDeclaration(e) => {
                let mut val = Value::Null;
                if !e.initializer.is_none() {
                    val = self.evaluate(e.initializer.unwrap())?;
                }

                self.environment.define(e.name.lexeme.clone(), val.clone());
                Ok(val.clone())
            }
        }
    }

    pub fn evaluate(&self, expression: Expression) -> Result<Value, InterpreterError> {
        match expression {
            Expression::Binary(b) => self.binary(b),
            Expression::Grouping(g) => self.grouping(g),
            Expression::Unary(u) => self.unary(u),
            Expression::Literal(l) => self.literal(l),
            Expression::Variable(v) => self.variable(v)
        }
    }

    pub fn execute(&mut self, statement: Statement) {
        self.statement(statement).unwrap();
    }

    pub fn interpret(&mut self, statements: Vec<Statement>) {
        for statement in statements {
            self.execute(statement);
        }
    }
}