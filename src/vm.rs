use std::{collections::HashMap, ffi::OsStr, process::Command};
//use std::os::windows::process;

use crate::{compiler::OpCode, main};

#[derive(Debug, Clone)]
pub enum ValueType<'a> {
    Number(f64),
    Str(&'a str),
    Boolean(bool),
    String(String),
}

impl ValueType<'_> {
    pub fn to_string(&self) -> String {
        match self {
            ValueType::Number(n) => format!("{n}"),
            ValueType::Boolean(b) => format!("{b}"),
            ValueType::Str(str) => str.to_string(),
            ValueType::String(str) => str.to_string(),
        }
    }
}

//impl Copy for ValueType<'_> {}

#[derive(Copy, Clone, Debug)]
struct Frame {
    //instructions: &'a Vec<OpCode>,
    ip: usize,
    frame_pointer: usize,
}

impl Frame {
    // pub fn current(&self) -> &OpCode {
    //     &self.instructions[self.ip]
    // }

    // pub fn inc(&mut self) -> bool {
    //     self.ip += 1;
    //     self.ip < self.instructions.len()
    // }
}

fn runtime_error(message: &str, line_number: u32) {
    eprintln!("runtime error: {message} in line {line_number}");
}

struct Function {
    pointer: usize,
    arity: i32,
}

pub struct Vm<'a> {
    stack: Vec<ValueType<'a>>,
    globals: HashMap<&'a String, ValueType<'a>>,
    //natives: Vec<fn(Vec<ValueType>) -> Result<ValueType, &str>>,
    natives: HashMap<&'a str, fn(Vec<ValueType>) -> Result<ValueType, &str>>,
    functions: HashMap<&'a str, Function>,
    pub return_value: Option<ValueType<'a>>,
}

fn print(params: Vec<ValueType>) -> Result<ValueType, &str> {
    if let Some(val) = params.first() {
        let s = val.to_string();
        println!("{s}");
        Result::Ok(ValueType::String(s))
    } else {
        Err("No parameters passed to function")
    }
}

fn system_command<'a>(
    command: &'a String,
    params: Vec<ValueType<'a>>,
) -> Result<ValueType<'a>, &'a str> {
    let mut args: Vec<String> = Vec::new();
    for param in params {
        args.push(param.to_string());
    }

    let output = Command::new(command)
        .args(args)
        .output()
        .expect("failed to execute process");

    let result = String::from_utf8_lossy(&output.stdout).to_string();

    Result::Ok(ValueType::String(result))
}

impl<'a> Vm<'a> {
    pub fn new() -> Self {
        Vm {
            stack: Vec::new(),
            globals: HashMap::new(),
            natives: HashMap::new(), //vec![print],
            functions: HashMap::new(),
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
            //instructions,
            ip: 0,
            frame_pointer: 0,
        };
        //call_frames.push(main_frame);
        let mut frame = main_frame;
        loop {
            //let frame = &mut call_frames[call_frames.len() - 1]; //  callFrames.last().unwrap(); //TODO: make safer
            //let mut frame = call_frames.last_mut().unwrap(); //TODO: make safer
            let instr = &instructions[frame.ip];
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
                    let mut args: Vec<ValueType> = Vec::new();

                    if let Some(func) = self.natives.get(&name.as_str()) {
                        for _i in 0..*argc {
                            let v = self.stack.pop().unwrap();
                            args.insert(0, v);
                        }
                        let result = func(args);
                        if let Ok(value) = result {
                            self.stack.push(value);
                        } else {
                            let message = format!("Error running {name}.");
                            runtime_error(&message, *line_number);
                            return false;
                        }
                    } else if let Some(func) = self.functions.get(&name.as_str()) {
                        // call user defined func
                        call_frames.push(frame); // save current frame
                        let argc = usize::try_from(*argc).unwrap();
                        frame = Frame {
                            ip: func.pointer - 1,
                            frame_pointer: self.stack.len() - argc,
                        };
                    } else {
                        let result = system_command(name, args);
                        if let Ok(value) = result {
                            self.stack.push(value);
                        } else {
                            let message = format!("Error running {name}.");
                            runtime_error(&message, *line_number);
                            return false;
                        }
                    }
                }
                OpCode::Pop => {
                    self.return_value = self.stack.pop();
                }
                OpCode::GetLocal(i, line_number) => {
                    self.stack.push(self.stack[i + frame.frame_pointer].clone());
                }
                OpCode::SetLocal(i, Line_number) => {
                    let value = self.stack.last().unwrap().clone();
                    self.stack[i + frame.frame_pointer] = value;
                }
                OpCode::JumpIfFalse(to_jump) => {
                    if let Some(result) = self.stack.pop() {
                        if let ValueType::Boolean(val) = result {
                            if !val {
                                frame.ip += to_jump;
                            }
                        }
                    }
                }
                OpCode::Jump(to_jump) => {
                    let current: i32 = frame.ip.try_into().unwrap();
                    let new_ip: usize = (current + to_jump).try_into().unwrap();
                    frame.ip = new_ip;
                }
                OpCode::Return => {
                    // pop the frame
                    // if no frames left, then break
                    if let Some(value) = call_frames.pop() {
                        frame = value;
                        let val = self.return_value.clone();
                        self.stack.push(val.unwrap());
                    } else {
                        break;
                    }
                    //panic!("return not implemented");
                }
                OpCode::DefFn(name, index, arity) => {
                    self.functions.insert(
                        name,
                        Function {
                            pointer: *index,
                            arity: *arity,
                        },
                    );
                    //panic!("DefFn not implemented");
                }
            }
            frame.ip += 1;
            if frame.ip >= instructions.len() {
                break;
            }
        }
        dbg!(&self.stack);

        true
    }
}
