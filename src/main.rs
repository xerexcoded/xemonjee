use std::collections::HashMap;

enum Command {
    SetVar(String, Value),
    GetVar(String),
    PushVar(String),
    Push(Value),
    Pop,
    Add,
}

#[derive(Clone, Debug, PartialEq)]
enum Value {
    Nothing,
    Int(i64),
    String(String),
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
    vars: HashMap<String, Value>,
    stack: Vec<Value>,
}
impl Evaluator {
    fn new() -> Evaluator {
        Self {
            vars: HashMap::new(),
            stack: vec![],
        }
    }
    fn pop(&mut self) -> Result<Value, EvalandParsingError> {
        let result = self.stack.pop(); // poping from the stack
        match result {
            Some(v) => Ok(v),
            None => return Err(EvalandParsingError::EmptyStack),
        }
    }
    fn add(&self, lhs: Value, rhs: Value) -> Result<Value, EvalandParsingError> {
        match (lhs, rhs) {
            (Value::Int(i1), Value::Int(i2)) => Ok(Value::Int(i1 + i2)),
            (Value::String(s1), Value::String(s2)) => Ok(Value::String(s1 + &s2)),
            _ => Err(EvalandParsingError::MismatchType),
        }
    }
    fn evaluate(&mut self, commands: &[Command]) -> Result<Value, EvalandParsingError> {
        let mut output = Ok(Value::Nothing); //initial state
        for command in commands {
            match command {
                Command::SetVar(name, value) => {
                    self.vars.insert(name.into(), value.clone());
                }
                Command::GetVar(name) => match self.vars.get(name) {
                    Some(value) => output = Ok(value.clone()),
                    None => return Err(EvalandParsingError::MissingVariable(name.into())),
                },
                Command::PushVar(name) => match self.vars.get(name) {
                    Some(value) => self.stack.push(value.clone()),
                    None => return Err(EvalandParsingError::MissingVariable(name.into())),
                },
                Command::Push(v) => self.stack.push(v.clone()),
                Command::Pop => {
                    output = self.pop();
                }
                Command::Add => {
                    let lhs = self.pop()?;
                    let rhs = self.pop()?;

                    let result = self.add(lhs, rhs)?;
                    self.stack.push(result)
                }
            }
        }
        output
    }
}

fn parse_var_name(var_name: &str) -> Result<String, EvalandParsingError> {
    Ok(var_name.into())
}
fn parse_string(val: &str) -> Result<Value, EvalandParsingError> {
    if val.starts_with("\"") && val.ends_with("\"") && val.len() > 1 {
        //>1 as to  avoid parsing quotes singular
        let inner = val[1..(val.len() - 1)].to_string(); // starting from 1 as 1st is "\""
        Ok(Value::String(inner))
    } else {
        Err(EvalandParsingError::MismatchType)
    }
}

fn parse_int(val: &str) -> Result<Value, EvalandParsingError> {
    let result = val.parse::<i64>();
    match result {
        Ok(x) => Ok(Value::Int(x)),
        _ => Err(EvalandParsingError::MismatchType),
    }
}
fn parse_value(val: &str) -> Result<Value, EvalandParsingError> {
    if val.starts_with("\"") && val.ends_with("\"") && val.len() > 1 {
        parse_string(val) // parse string
    } else {
        parse_int(val) // parse number
    }
}
fn parse_set(input: &[&str]) -> Result<Command, EvalandParsingError> {
    if input.len() != 3 {
        // must have exactly 3 parameters , well I am learning still so🦀
        return Err(EvalandParsingError::MismatchNumParams);
    }
    let var_name = parse_var_name(input[1])?; //indexing from 1 as o is the binary itself
    let value = parse_value(input[2])?;

    Ok(Command::SetVar(var_name, value))
}
fn parse_get(input: &[&str]) -> Result<Command, EvalandParsingError> {
    if input.len() != 2 {
        return Err(EvalandParsingError::MismatchNumParams);
    }
    let var_name = parse_var_name(input[1])?;
    Ok(Command::GetVar(var_name))
}
fn parse_pushvar(input: &[&str]) -> Result<Command, EvalandParsingError> {
    if input.len() != 2 {
        return Err(EvalandParsingError::MismatchNumParams);
    }
    let var_name = parse_var_name(input[1])?;
    Ok(Command::PushVar(var_name))
}

fn parse_push(input: &[&str]) -> Result<Command, EvalandParsingError> {
    if input.len() != 2 {
        return Err(EvalandParsingError::MismatchNumParams);
    }
    let val = parse_value(input[1])?;
    Ok(Command::Push(val))
}

fn parse(input: &str) -> Result<Vec<Command>, EvalandParsingError> {
    let mut op = vec![];
    for line in input.lines() {
        let command: Vec<_> = line.split_ascii_whitespace().collect(); // collect into vector
        match command.get(0) {
            Some(x) if *x == "set" => {
                op.push(parse_set(&command)?);
            }
            Some(x) if *x == "get" => {
                op.push(parse_get(&command)?);
            }
            Some(x) if *x == "push" => {
                op.push(parse_push(&command)?);
            }
            Some(x) if *x == "pushvar" => {
                op.push(parse_pushvar(&command)?);
            }
            Some(x) if *x == "pop" => {
                op.push(Command::Pop);
            }
            Some(x) if *x == "add" => {
                op.push(Command::Add);
            }
            Some(name) => return Err(EvalandParsingError::UnknownCommand(name.to_string())),
            None => {} // do Nothing
        }
    }
    Ok(op)
}

// the main show finally🦀
fn main() -> Result<(), EvalandParsingError> {
    // returns unit or EvalandPArsingError
    for args in std::env::args().skip(1) {
        let contents = std::fs::read_to_string(args).unwrap();
        let mut interpreter = Evaluator::new();
        let coms = parse(&contents)?;
        let ans = interpreter.evaluate(&coms)?;

        println!("{:?}", ans);
    }
    Ok(())
}
