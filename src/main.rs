use std::collections::HashMap;

enum Command {
    SetVar(String , Value),
    Getvar(String),
    PushVar(String),
    Push(Value),
    Pop,
    Add
}

#[derive(Clone , Debug , PartialEq)]
enum Value {
    Nothing,
    Int(64),
    String(String),
}

#[derive(Clone , Debug , PartialEq)]
enum Type {
    Nothing,
    Int,
    String,
}

#[derive(Debug)]
enum EvalandParsingError {
    MismatchNumParams, //need exact number  of parameters to work :]
    MismatchType,
    UnknownCommand(String),
    MissingVariable(String),
    EmptyStack,
}

struct Evaluator {
    fn new() -> Evaluator {
        Self {
            vars : HashMap::new(),
            stack: vec![],
        }
    }
    fn pop(&mut self) -> Result<Value, EvalandParsingError> {
        let result = self.stack.pop(); // poping from the stack 
        match result {
            Some(V) => Ok(v),
            None => return Err(EvalandParsingError::EmptyStack),
        }
    }
    fn add(&self, lhs: value ,rhs:value)-> Result<Value, EvalandParsingError>{
        match(lhs,rhs) {
            (Value::Int(i1),Value::Int(i2)) => Ok(Value::Int(i1+i2)),
            (Value::String(s1),Value::String(s2)) => Ok(Value:String(s1+ &s2)),
            _ => Err(EvalandParsingError::MismatchType),
        }
    }
    fn 
    
}


fn main() {
    
}















