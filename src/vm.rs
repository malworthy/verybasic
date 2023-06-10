use std::collections::HashMap;
//use std::os::windows::process;

use crate::{compiler::OpCode, main};

#[derive(Debug, Clone)]
pub enum ValueType<'a> {
    Number(f64),
    Str(&'a str),
    Boolean(bool),
    String(String),
}

//impl Copy for ValueType<'_> {}

#[derive(Debug)]
struct Frame<'a> {
    instructions: &'a Vec<OpCode>,
    ip: usize,
    frame_pointer: usize,
}

impl Frame<'_> {
    pub fn current(&self) -> &OpCode {
        &self.instructions[self.ip]
    }

    pub fn inc(&mut self) -> bool {
        self.ip += 1;
        self.ip < self.instructions.len()
    }
}

fn runtime_error(message: &str, line_number: u32) {
    eprintln!("runtime error: {message} in line {line_number}");
}

pub struct Vm<'a> {
    stack: Vec<ValueType<'a>>,
    globals: HashMap<&'a String, ValueType<'a>>,
    //natives: Vec<fn(Vec<ValueType>) -> Result<ValueType, &str>>,
    natives: HashMap<&'a str, fn(Vec<ValueType>) -> Result<ValueType, &str>>,
    pub return_value: Option<ValueType<'a>>,
}

fn print(params: Vec<ValueType>) -> Result<ValueType, &str> {
    if let Some(val) = params.first() {
        let s = match val {
            ValueType::Number(n) => format!("{n}"),
            ValueType::Boolean(b) => format!("{b}"),
            ValueType::Str(str) => str.to_string(),
            ValueType::String(str) => str.to_string(),
        };
        println!("{s}");
        Result::Ok(ValueType::String(s))
    } else {
        Err("No parameters passed to function")
    }
}

impl<'a> Vm<'a> {
    pub fn new() -> Self {
        Vm {
            stack: Vec::new(),
            globals: HashMap::new(),
            natives: HashMap::new(), //vec![print],
            return_value: Option::None,
        }
    }

    pub fn init(&mut self) {
        self.natives.insert("print", print);
    }

    fn comparison(&mut self, op: &OpCode, line_number: u32) -> bool {
        let b = self.stack.pop().unwrap();
        let a = self.stack.pop().unwrap();
        let result = match a {
            ValueType::Number(a) => {
                if let ValueType::Number(b) = b {
                    match op {
                        OpCode::GreaterThan(_) => ValueType::Boolean(a > b),
                        OpCode::GreaterThanEq(_) => ValueType::Boolean(a >= b),
                        OpCode::LessThan(_) => ValueType::Boolean(a < b),
                        OpCode::LessThanEq(_) => ValueType::Boolean(a <= b),
                        OpCode::Equal(_) => ValueType::Boolean(a == b),
                        OpCode::NotEqual(_) => ValueType::Boolean(a != b),
                        _ => panic!("Non-comparison opcode processed in comparison()"),
                    }
                } else {
                    runtime_error("type mismatch", line_number);
                    return false;
                }
            }
            ValueType::Str(a) => panic!("String comparison not yet implemented"),
            ValueType::String(a) => panic!("String comparison not yet implemented"),
            ValueType::Boolean(_) => {
                runtime_error("boolean not valid for comparison operation", line_number);
                return false;
            }
        };

        self.stack.push(result);
        true
    }

    fn binary(&mut self, op: &OpCode, line_number: u32) -> bool {
        let b = self.stack.pop().unwrap();
        let a = self.stack.pop().unwrap();
        let result = match a {
            ValueType::Number(a) => {
                if let ValueType::Number(b) = b {
                    match op {
                        OpCode::Subtract(_) => ValueType::Number(a - b),
                        OpCode::Multiply(_) => ValueType::Number(a * b),
                        OpCode::Divide(_) => ValueType::Number(a / b),
                        _ => panic!("Non-binary opcode processed in binary()"),
                    }
                } else {
                    runtime_error("type mismatch", line_number);
                    return false;
                }
            }
            _ => {
                runtime_error("type mismatch", line_number);
                return false;
            }
        };

        self.stack.push(result);
        true
    }

    fn negate(&mut self, line_number: u32) -> bool {
        let a = self.stack.pop().unwrap();
        let result = match a {
            ValueType::Number(a) => ValueType::Number(-a),
            _ => {
                runtime_error(
                    "Type mismatch. '-' can only be used on numbers.",
                    line_number,
                );
                return false;
            }
        };

        self.stack.push(result);
        true
    }

    fn not(&mut self, line_number: u32) -> bool {
        let a = self.stack.pop().unwrap();
        let result = match a {
            ValueType::Number(a) => ValueType::Boolean(a == 0.0),
            ValueType::Boolean(a) => ValueType::Boolean(!a),
            ValueType::Str(a) => ValueType::Boolean(a.len() == 0),
            ValueType::String(a) => ValueType::Boolean(a.len() == 0),
        };

        self.stack.push(result);
        true
    }

    fn add(&mut self, op: &OpCode, line_number: u32) -> bool {
        let b = self.stack.pop().unwrap();
        let a = self.stack.pop().unwrap();
        let result = match a {
            ValueType::Number(a) => {
                if let ValueType::Number(b) = b {
                    ValueType::Number(a + b)
                } else {
                    runtime_error("type mismatch", line_number);
                    return false;
                }
            }
            ValueType::Str(a) => {
                let joined = match b {
                    ValueType::Str(b) => format!("{}{}", a, b),
                    ValueType::String(b) => format!("{}{}", a, b),
                    _ => {
                        runtime_error("type mismatch", line_number);
                        return false;
                    }
                };
                ValueType::String(joined.clone())
            }
            ValueType::String(a) => {
                let joined = match b {
                    ValueType::Str(b) => format!("{}{}", a, b),
                    ValueType::String(b) => format!("{}{}", a, b),
                    _ => {
                        runtime_error("type mismatch", line_number);
                        return false;
                    }
                };
                ValueType::String(joined.clone())
            }
            ValueType::Boolean(_) => {
                runtime_error("Cannot add a boolean", line_number);
                return false;
            }
        };

        self.stack.push(result);
        true
    }

    pub fn run(&mut self, instructions: &'a Vec<OpCode>) -> bool {
        let mut call_frames: Vec<Frame> = Vec::new();
        //let mut stack: Vec<ValueType> = Vec::new();
        let main_frame = Frame {
            instructions,
            ip: 0,
            frame_pointer: 0,
        };
        call_frames.push(main_frame);
        loop {
            //let frame = &mut call_frames[call_frames.len() - 1]; //  callFrames.last().unwrap(); //TODO: make safer
            let frame = &mut call_frames.last_mut().unwrap(); //TODO: make safer
            let instr = &frame.instructions[frame.ip];
            match instr {
                OpCode::ConstantNum(num, _) => {
                    self.stack.push(ValueType::Number(*num));
                }
                OpCode::ConstantStr(str, _) => {
                    self.stack.push(ValueType::Str(str));
                }
                OpCode::Subtract(line_number) => {
                    if !self.binary(&instr, *line_number) {
                        return false;
                    };
                }
                OpCode::Multiply(line_number) => {
                    if !self.binary(&instr, *line_number) {
                        return false;
                    };
                }
                OpCode::Divide(line_number) => {
                    if !self.binary(&instr, *line_number) {
                        return false;
                    };
                }
                OpCode::Add(line_number) => {
                    if !self.add(&instr, *line_number) {
                        return false;
                    };
                }
                OpCode::Negate(line_number) => {
                    if !self.negate(*line_number) {
                        return false;
                    };
                }
                OpCode::Not(line_number) => {
                    if !self.not(*line_number) {
                        return false;
                    };
                }
                OpCode::GreaterThan(line_number) => {
                    if !self.comparison(&instr, *line_number) {
                        return false;
                    };
                }
                OpCode::GreaterThanEq(line_number) => {
                    if !self.comparison(&instr, *line_number) {
                        return false;
                    };
                }
                OpCode::LessThan(line_number) => {
                    if !self.comparison(&instr, *line_number) {
                        return false;
                    };
                }
                OpCode::LessThanEq(line_number) => {
                    if !self.comparison(&instr, *line_number) {
                        return false;
                    };
                }
                OpCode::Equal(line_number) => {
                    if !self.comparison(&instr, *line_number) {
                        return false;
                    };
                }
                OpCode::NotEqual(line_number) => {
                    if !self.comparison(&instr, *line_number) {
                        return false;
                    };
                }
                OpCode::SetGlobal(name, line_number) => {
                    let v = self.stack.last().unwrap();
                    self.globals.insert(name, v.clone());
                }
                OpCode::GetGlobal(name, line_number) => {
                    if let Some(value) = self.globals.get(&name) {
                        self.stack.push(value.clone());
                    } else {
                        let message = format!("Global variable {name} does not exist.");
                        runtime_error(&message, *line_number);
                        return false;
                    }
                }
                OpCode::Call(name, argc, line_number) => {
                    println!("calling function {name}");
                    let mut args: Vec<ValueType> = Vec::new();
                    for _i in 0..*argc {
                        let v = self.stack.pop().unwrap();
                        args.insert(0, v);
                        //args.push(v);
                    }
                    if let Some(func) = self.natives.get(&name.as_str()) {
                        let result = func(args);
                        if let Ok(value) = result {
                            self.stack.push(value);
                        } else {
                            let message = format!("Function {name} does not exist.");
                            runtime_error(&message, *line_number);
                            return false;
                        }
                    }
                }
                OpCode::Pop => {
                    self.return_value = self.stack.pop();
                }
            }

            if !frame.inc() {
                break;
            }
        }
        dbg!(&self.stack);

        true
    }
}
