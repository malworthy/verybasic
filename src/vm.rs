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
    String(&'a str),
    Boolean(bool),
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
fn binary(stack: &mut Vec<ValueType>, op: &OpCode) -> bool {
    let b = stack.pop().unwrap();
    let a = stack.pop().unwrap();
    let result = match a {
        ValueType::Number(a) => {
            if let ValueType::Number(b) = b {
                ValueType::Number(a + b)
            } else {
                runtime_error("type mismatch", 0);
                return false;
            }
        }
        _ => {
            runtime_error("Not implemented", 0);
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
            OpCode::ConstantNum(num) => stack.push(ValueType::Number(*num)),
            OpCode::ConstantStr(str) => stack.push(ValueType::String(&str)),
            OpCode::Add => {
                binary(&mut stack, &instr);
            }
        }

        if !frame.inc() {
            break;
        }
    }
    dbg!(stack);
}
