use std::os::windows::process;

use crate::{compiler::OpCode, main};

#[derive(Debug)]
struct Value<'a> {
    number: f64,
    string: &'a str,
    boolean: bool,
}

#[derive(Debug)]
enum ValueType<'a> {
    Number(f64),
    Str(&'a str),
    Boolean(bool),
    String(String),
}

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

fn subtract(stack: &mut Vec<ValueType>, op: &OpCode, line_number: u32) -> bool {
    let b = stack.pop().unwrap();
    let a = stack.pop().unwrap();
    let result = match a {
        ValueType::Number(a) => {
            if let ValueType::Number(b) = b {
                ValueType::Number(a - b)
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

    stack.push(result);
    true
}

fn negate(stack: &mut Vec<ValueType>, line_number: u32) -> bool {
    let a = stack.pop().unwrap();
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

    stack.push(result);
    true
}

fn add(stack: &mut Vec<ValueType>, op: &OpCode, line_number: u32) -> bool {
    let b = stack.pop().unwrap();
    let a = stack.pop().unwrap();
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

    stack.push(result);
    true
}

pub fn runtime_error(message: &str, line_number: u32) {
    eprintln!("runtime error: {message} in line {line_number}");
    std::process::exit(1);
}

pub fn run(instructions: &Vec<OpCode>) {
    let mut call_frames: Vec<Frame> = Vec::new();
    let mut stack: Vec<ValueType> = Vec::new();
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
            OpCode::ConstantNum(num, _) => stack.push(ValueType::Number(*num)),
            OpCode::ConstantStr(str, _) => stack.push(ValueType::Str(str)),
            OpCode::Subtract(line_number) => {
                subtract(&mut stack, &instr, *line_number);
            }
            OpCode::Add(line_number) => {
                add(&mut stack, &instr, *line_number);
            }
            OpCode::Negate(line_number) => {
                negate(&mut stack, *line_number);
            }
        }

        if !frame.inc() {
            break;
        }
    }
    dbg!(stack);
}
