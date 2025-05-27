use std::collections::HashMap;
use crate::interpreter::{Value, InterpreterError};
use crate::token::Token;


pub struct Environment {
    pub values: HashMap<String, Value> 
}

impl Environment {
    pub fn new() -> Self {
        Self {
            values: HashMap::new()
        }
    }
    pub fn define(&mut self, name: String, value: Value) {
        self.values.insert(name, value);
    }

    pub fn get(&self, token: Token) -> Result<Value, InterpreterError>  {
        match self.values.get(&token.lexeme) {
            Some(val) => Ok(val.clone()),
            None => {let name = &token.lexeme; println!("Undefined variable '{name}'."); Err(InterpreterError)}
        }
    }
}