mod functions;
mod graphics;
use std::{collections::HashMap, process::Command};

use colored::Colorize;

use crate::compiler::OpCode;

#[derive(Debug, Clone)]
pub enum ValueType<'a> {
    Number(f64),
    Str(&'a str),
    Boolean(bool),
    String(String),
    Array(Vec<ValueType<'a>>),
}

impl ValueType<'_> {
    pub fn to_string(&self) -> String {
        match self {
            ValueType::Number(n) => format!("{n}"),
            ValueType::Boolean(b) => format!("{b}"),
            ValueType::Str(str) => str.to_string(),
            ValueType::String(str) => str.to_string(),
            ValueType::Array(a) => format!("{:?}", a),
        }
    }
}

#[derive(Copy, Clone, Debug)]
struct Frame {
    ip: usize,
    frame_pointer: usize,
}

fn runtime_error(message: &str, line_number: u32) {
    eprintln!("Runtime error: {} in line {line_number}", message.red());
}

pub struct Vm<'a> {
    //stack: Vec<ValueType<'a>>,
    stack: [ValueType<'a>; 256],
    stack_pointer: usize,
    globals: HashMap<&'a String, ValueType<'a>>,
    pub return_value: Option<ValueType<'a>>,
    gr: graphics::Graphics,
}

fn system_command<'a>(
    command: &'a String,
    params: Vec<ValueType<'a>>,
) -> Result<ValueType<'a>, &'a str> {
    let mut args: Vec<String> = Vec::new();
    for param in params {
        args.push(param.to_string());
    }

    let first_char = command.chars().nth(0).unwrap();

    let output = if first_char == '@' {
        let command = &command[1..];
        Command::new(command).args(args).output()
    } else {
        Command::new(command).args(args).output()
    };

    if let Ok(output) = output {
        let result = String::from_utf8_lossy(&output.stdout).to_string();
        Result::Ok(ValueType::String(result))
    } else {
        Result::Err("Failed to run system command")
    }
}

fn string_compare<'a>(op: &OpCode, a: &str, b: &str) -> ValueType<'a> {
    match op {
        OpCode::GreaterThan(_) => ValueType::Boolean(a > b),
        OpCode::GreaterThanEq(_) => ValueType::Boolean(a >= b),
        OpCode::LessThan(_) => ValueType::Boolean(a < b),
        OpCode::LessThanEq(_) => ValueType::Boolean(a <= b),
        OpCode::Equal(_) => ValueType::Boolean(a == b),
        OpCode::NotEqual(_) => ValueType::Boolean(a != b),
        _ => panic!("Non-comparison opcode processed in comparison()"),
    }
}

const EMPTY_ELEMENT: ValueType = ValueType::Boolean(false);

macro_rules! pop {
    ($s:ident, $v:ident) => {
        $s.stack_pointer -= 1;
        let $v = &$s.stack[$s.stack_pointer];
    };
}

impl<'a> Vm<'a> {
    pub fn new() -> Self {
        Vm {
            //stack: Vec::new(),
            stack: [EMPTY_ELEMENT; 256],
            globals: HashMap::new(),
            return_value: Option::None,
            gr: graphics::Graphics::new(),
            stack_pointer: 0,
        }
    }

    pub const NATIVES: [(fn(Vec<ValueType>) -> Result<ValueType, &str>, &str); 9] = [
        (functions::print, "print"),
        (functions::input, "input"),
        (functions::array, "array"),
        (functions::len, "len"),
        (functions::seconds, "seconds"),
        (functions::dir, "dir"),
        (functions::readlines, "readlines"),
        (functions::random, "rand"),
        (functions::rgb, "rgb"),
    ];

    pub const NATIVES_GR: [(
        fn(Vec<ValueType<'a>>, &'a mut graphics::Graphics) -> Result<ValueType<'a>, &'a str>,
        &str,
    ); 4] = [
        (functions::window, "window"),
        (functions::plot, "plot"),
        (functions::clear_graphics, "cleargraphics"),
        (functions::init_graphics, "initgraphics"),
    ];

    fn push(&mut self, value: ValueType<'a>) {
        if self.stack_pointer > 255 {
            runtime_error("Stack Overflow", 0);
        }
        self.stack[self.stack_pointer] = value;
        self.stack_pointer += 1;
    }

    fn comparison(&mut self, op: &OpCode, line_number: u32) -> bool {
        pop!(self, b);
        pop!(self, a);

        let result = match a {
            ValueType::Number(a) => {
                if let ValueType::Number(ref b) = b {
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
            ValueType::Str(a) => match b {
                ValueType::Str(b) => string_compare(&op, a, b), // ValueType::Boolean(a == b),
                ValueType::String(b) => string_compare(&op, a, b.as_str()), //ValueType::Boolean(a == b),
                _ => {
                    runtime_error("You cannot compare a string to a non-string", line_number);
                    return false;
                }
            },
            ValueType::String(a) => match b {
                ValueType::Str(b) => string_compare(&op, a.as_str(), b), //ValueType::Boolean(a == b),
                ValueType::String(b) => string_compare(&op, a.as_str(), b.as_str()), //ValueType::Boolean(a == b),
                _ => {
                    runtime_error("You cannot compare a string to a non-string.", line_number);
                    return false;
                }
            },
            ValueType::Boolean(_) => {
                runtime_error("Boolean not valid for comparison operation", line_number);
                return false;
            }
            ValueType::Array(_) => {
                runtime_error("Array not valid for comparison operation", line_number);
                return false;
            }
        };

        self.push(result);
        true
    }

    fn binary(&mut self, op: &OpCode, line_number: u32) -> bool {
        pop!(self, b);
        pop!(self, a);

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

        self.push(result);
        true
    }

    fn and_or(&mut self, op: &OpCode, line_number: u32) -> bool {
        pop!(self, b);
        pop!(self, a);

        let result = match a {
            ValueType::Boolean(a) => {
                if let ValueType::Boolean(b) = b {
                    match op {
                        OpCode::And(_) => ValueType::Boolean(*a && *b),
                        OpCode::Or(_) => ValueType::Boolean(*a || *b),
                        _ => panic!("And/Or opcode expected"),
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

        self.push(result);
        true
    }

    fn negate(&mut self, line_number: u32) -> bool {
        pop!(self, a);

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

        self.push(result);
        true
    }

    fn not(&mut self, line_number: u32) -> bool {
        pop!(self, a);

        let result = match a {
            ValueType::Number(a) => ValueType::Boolean(*a == 0.0),
            ValueType::Boolean(a) => ValueType::Boolean(!a),
            ValueType::Str(a) => ValueType::Boolean(a.len() == 0),
            ValueType::String(a) => ValueType::Boolean(a.len() == 0),
            ValueType::Array(_) => {
                runtime_error("not invalid for an array", line_number);
                return false;
            }
        };

        self.push(result);
        true
    }

    fn add(&mut self, line_number: u32) -> bool {
        pop!(self, b);
        pop!(self, a);

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
            ValueType::Array(_) => {
                runtime_error("Cannot add an array", line_number);
                return false;
            }
        };

        self.push(result);
        true
    }

    pub fn run(&mut self, instructions: &'a Vec<OpCode>) -> bool {
        //dbg!(&instructions);
        let mut call_frames: Vec<Frame> = Vec::new();
        let main_frame = Frame {
            ip: 0,
            frame_pointer: 0,
        };
        let mut frame = main_frame;
        loop {
            let instr = &instructions[frame.ip];
            match instr {
                OpCode::ConstantNum(num, _) => {
                    self.push(ValueType::Number(*num));
                }
                OpCode::ConstantStr(str, _) => {
                    self.push(ValueType::Str(str));
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
                    if !self.add(*line_number) {
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
                OpCode::SetGlobal(name) => {
                    let v = &self.stack[self.stack_pointer - 1];
                    self.globals.insert(name, v.clone());
                }
                OpCode::GetGlobal(name, line_number) => {
                    if let Some(value) = self.globals.get(&name) {
                        self.push(value.clone());
                    } else {
                        let message = format!("Global variable {name} does not exist.");
                        runtime_error(&message, *line_number);
                        return false;
                    }
                }
                OpCode::CallNative(index, argc, line_number) => {
                    let mut args: Vec<ValueType> = Vec::new();

                    let func = Vm::NATIVES[*index].0;
                    // call a native/built-in function
                    for _i in 0..*argc {
                        // self.stack_pointer -= 1;
                        // let v = &self.stack[self.stack_pointer];
                        pop!(self, v);
                        args.insert(0, v.clone());
                    }
                    let result = func(args);

                    match result {
                        Ok(value) => self.push(value),
                        Err(message) => {
                            runtime_error(&message, *line_number);
                            return false;
                        }
                    }
                }
                OpCode::CallNativeGr(index, argc, line_number) => {
                    let mut args: Vec<ValueType> = Vec::new();
                    let func = Vm::NATIVES_GR[*index].0;

                    // call a native/built-in function
                    for _i in 0..*argc {
                        pop!(self, v);
                        args.insert(0, v.clone());
                    }
                    if let Err(msg) = func(args, &mut self.gr) {
                        runtime_error(msg, *line_number);
                        return false;
                    }
                    self.push(ValueType::Boolean(true));
                }
                OpCode::CallSystem(name, argc, line_number) => {
                    let mut args: Vec<ValueType> = Vec::new();
                    for _i in 0..*argc {
                        pop!(self, v);
                        args.insert(0, v.clone());
                    }

                    let result = system_command(name, args);

                    match result {
                        Ok(value) => self.push(value),
                        Err(message) => {
                            runtime_error(&message, *line_number);
                            return false;
                        }
                    }
                }
                OpCode::Call(pointer, argc) => {
                    call_frames.push(frame); // save current frame
                    let argc = *argc as usize;
                    frame = Frame {
                        ip: pointer - 1,
                        frame_pointer: self.stack_pointer - argc,
                    };
                }
                OpCode::Pop => {
                    pop!(self, v);
                    self.return_value = Some(v.clone());
                }
                OpCode::GetLocal(i) => {
                    self.push(self.stack[i + frame.frame_pointer].clone());
                }
                OpCode::SetLocal(i) => {
                    let value = self.stack[self.stack_pointer - 1].clone();
                    self.stack[i + frame.frame_pointer] = value;
                }
                OpCode::DefineLocal => {
                    let value = self.stack[self.stack_pointer - 1].clone();
                    self.push(value);
                }
                // do a define local
                OpCode::JumpIfFalse(to_jump) => {
                    pop!(self, result);
                    //if let Some(result) = self.stack.pop() {
                    if let ValueType::Boolean(val) = result {
                        if !val {
                            frame.ip += to_jump;
                        }
                    }
                    //}
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
                        // get rid of any local variables on the stack
                        self.stack_pointer = frame.frame_pointer;
                        //self.stack.truncate(frame.frame_pointer);
                        // set the call frame
                        frame = value;
                        let val = self.return_value.clone();
                        self.push(val.unwrap());
                    } else {
                        break;
                    }
                }
                OpCode::And(line_number) | OpCode::Or(line_number) => {
                    if !self.and_or(instr, *line_number) {
                        return false;
                    }
                }
                OpCode::Subscript(line_number) => {
                    // let index = self.stack.pop().unwrap();
                    // let array = self.stack.pop().unwrap();
                    pop!(self, index);
                    pop!(self, array);

                    if let ValueType::Array(a) = array {
                        if let ValueType::Number(index) = index {
                            let i = *index as usize;
                            if let Some(val) = a.get(i) {
                                self.push(val.clone());
                            } else {
                                runtime_error("Subscript out of range", *line_number);
                                return false;
                            }
                        } else {
                            runtime_error("Subscript index must be a number", *line_number);
                            return false;
                        }
                    } else {
                        runtime_error("Subscript only works on arrays", *line_number);
                        return false;
                    }
                }
            }
            frame.ip += 1;
            if frame.ip >= instructions.len() {
                break;
            }
        }
        //dbg!(&self.stack);

        true
    }
}
