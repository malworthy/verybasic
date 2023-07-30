mod functions;
mod graphics;
mod string_functions;
use std::{collections::HashMap, path::PathBuf, process::Command};

use crate::compiler::OpCode;
use colored::Colorize;

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

const MAX_STACK: usize = 512;

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
        OpCode::GreaterThan => ValueType::Boolean(a > b),
        OpCode::GreaterThanEq => ValueType::Boolean(a >= b),
        OpCode::LessThan => ValueType::Boolean(a < b),
        OpCode::LessThanEq => ValueType::Boolean(a <= b),
        OpCode::Equal => ValueType::Boolean(a == b),
        OpCode::NotEqual => ValueType::Boolean(a != b),
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

pub struct Vm<'a> {
    stack: [ValueType<'a>; MAX_STACK],
    stack_pointer: usize,
    globals: HashMap<&'a String, ValueType<'a>>,
    pub return_value: Option<ValueType<'a>>,
    gr: graphics::Graphics,
    line_numbers: &'a mut Vec<u32>,
    ip: usize,
    pub config_file: PathBuf,
}

impl<'a> Vm<'a> {
    pub fn new(line_numbers: &'a mut Vec<u32>) -> Self {
        Vm {
            //stack: Vec::new(),
            stack: [EMPTY_ELEMENT; MAX_STACK],
            globals: HashMap::new(),
            return_value: Option::None,
            gr: graphics::Graphics::new(),
            stack_pointer: 0,
            ip: 0,
            line_numbers,
            config_file: PathBuf::from("settings.txt"),
        }
    }
    //fn rgb<'a>(params: Vec<ValueType<'a>>, _: &Vm<'a>) -> Result<ValueType<'a>, &'a str>
    pub const NATIVES: [(
        fn(Vec<ValueType<'a>>, &mut Vm<'a>) -> Result<ValueType<'a>, &'a str>,
        &str,
    ); 32] = [
        (functions::print, "print"),
        (functions::input, "input"),
        (functions::array, "array"),
        (functions::len, "len"),
        (functions::seconds, "seconds"),
        (functions::dir, "dir"),
        (functions::readlines, "readlines"),
        (functions::random, "rand"),
        (functions::rgb, "rgb"),
        (string_functions::mid, "mid"),
        (string_functions::left, "left"),
        (functions::floor, "floor"),
        (string_functions::str, "str"),
        (functions::write, "write"),
        (functions::append, "append"),
        (functions::chr, "chr"),
        (functions::val, "val"),
        (string_functions::right, "right"),
        (string_functions::ucase, "ucase"),
        (string_functions::lcase, "lcase"),
        (string_functions::instr, "instr"),
        (functions::command, "command"),
        (functions::now, "now"),
        (functions::window, "window"),
        (functions::plot, "plot"),
        (functions::clear_graphics, "cleargraphics"),
        (functions::init_graphics, "initgraphics"),
        (functions::setting_set, "setting_set"),
        (functions::setting_get, "setting_get"),
        (functions::stack, "stack"),
        (functions::sort, "sort"),
        (functions::push, "push"),
    ];

    pub fn debug_stack(&mut self) {
        dbg!(&self.stack[0..self.stack_pointer + 1]);
    }

    // pub const NATIVES_GR: [(
    //     fn(Vec<ValueType<'a>>, &'a mut graphics::Graphics) -> Result<ValueType<'a>, &'a str>,
    //     &str,
    // ); 3] = [];

    fn runtime_error(&mut self, message: &str) {
        let line_number = self.line_numbers[self.ip];
        eprintln!("Runtime error: {} in line {line_number}", message.red());
    }

    fn push(&mut self, value: ValueType<'a>) {
        if self.stack_pointer >= MAX_STACK {
            self.runtime_error("Stack Overflow");
        }
        self.stack[self.stack_pointer] = value;
        self.stack_pointer += 1;
    }

    fn comparison(&mut self, op: &OpCode) -> bool {
        pop!(self, b);
        pop!(self, a);

        let result = match a {
            ValueType::Number(a) => {
                if let ValueType::Number(ref b) = b {
                    match op {
                        OpCode::GreaterThan => ValueType::Boolean(a > b),
                        OpCode::GreaterThanEq => ValueType::Boolean(a >= b),
                        OpCode::LessThan => ValueType::Boolean(a < b),
                        OpCode::LessThanEq => ValueType::Boolean(a <= b),
                        OpCode::Equal => ValueType::Boolean(a == b),
                        OpCode::NotEqual => ValueType::Boolean(a != b),
                        _ => panic!("Non-comparison opcode processed in comparison()"),
                    }
                } else {
                    self.runtime_error("type mismatch");
                    return false;
                }
            }
            ValueType::Str(a) => match b {
                ValueType::Str(b) => string_compare(&op, a, b), // ValueType::Boolean(a == b),
                ValueType::String(b) => string_compare(&op, a, b.as_str()), //ValueType::Boolean(a == b),
                _ => {
                    self.runtime_error("You cannot compare a string to a non-string");
                    return false;
                }
            },
            ValueType::String(a) => match b {
                ValueType::Str(b) => string_compare(&op, a.as_str(), b), //ValueType::Boolean(a == b),
                ValueType::String(b) => string_compare(&op, a.as_str(), b.as_str()), //ValueType::Boolean(a == b),
                _ => {
                    self.runtime_error("You cannot compare a string to a non-string.");
                    return false;
                }
            },
            ValueType::Boolean(_) => {
                self.runtime_error("Boolean not valid for comparison operation");
                return false;
            }
            ValueType::Array(_) => {
                self.runtime_error("Array not valid for comparison operation");
                return false;
            }
        };

        self.push(result);
        true
    }

    fn binary(&mut self, op: &OpCode) -> bool {
        pop!(self, b);
        pop!(self, a);

        let result = match a {
            ValueType::Number(a) => {
                if let ValueType::Number(b) = b {
                    match op {
                        OpCode::Subtract => ValueType::Number(a - b),
                        OpCode::Multiply => ValueType::Number(a * b),
                        OpCode::Divide => ValueType::Number(a / b),
                        OpCode::Pow => ValueType::Number(a.powf(*b)),
                        OpCode::Mod => ValueType::Number(a % b),
                        _ => panic!("Non-binary opcode processed in binary()"),
                    }
                } else {
                    self.runtime_error("type mismatch");
                    return false;
                }
            }
            _ => {
                self.runtime_error("type mismatch");
                return false;
            }
        };

        self.push(result);
        true
    }

    fn and_or(&mut self, op: &OpCode) -> bool {
        pop!(self, b);
        pop!(self, a);

        let result = match a {
            ValueType::Boolean(a) => {
                if let ValueType::Boolean(b) = b {
                    match op {
                        OpCode::And => ValueType::Boolean(*a && *b),
                        OpCode::Or => ValueType::Boolean(*a || *b),
                        _ => panic!("And/Or opcode expected"),
                    }
                } else {
                    self.runtime_error("type mismatch");
                    return false;
                }
            }
            _ => {
                self.runtime_error("type mismatch");
                return false;
            }
        };

        self.push(result);
        true
    }

    fn negate(&mut self) -> bool {
        pop!(self, a);

        let result = match a {
            ValueType::Number(a) => ValueType::Number(-a),
            _ => {
                self.runtime_error("Type mismatch. '-' can only be used on numbers.");
                return false;
            }
        };

        self.push(result);
        true
    }

    fn not(&mut self) -> bool {
        pop!(self, a);

        let result = match a {
            ValueType::Number(a) => ValueType::Boolean(*a == 0.0),
            ValueType::Boolean(a) => ValueType::Boolean(!a),
            ValueType::Str(a) => ValueType::Boolean(a.len() == 0),
            ValueType::String(a) => ValueType::Boolean(a.len() == 0),
            ValueType::Array(_) => {
                self.runtime_error("not invalid for an array");
                return false;
            }
        };

        self.push(result);
        true
    }

    fn add(&mut self) -> bool {
        pop!(self, b);
        pop!(self, a);

        //println!("Add {:?} + {:?}", a, b);

        let result = match a {
            ValueType::Number(a) => {
                if let ValueType::Number(b) = b {
                    ValueType::Number(a + b)
                } else {
                    self.runtime_error("type mismatch");
                    return false;
                }
            }
            ValueType::Str(a) => {
                let joined = match b {
                    ValueType::Str(b) => format!("{}{}", a, b),
                    ValueType::String(b) => format!("{}{}", a, b),
                    _ => {
                        self.runtime_error("type mismatch");
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
                        self.runtime_error("type mismatch");
                        return false;
                    }
                };
                ValueType::String(joined.clone())
            }
            ValueType::Boolean(_) => {
                self.runtime_error("Cannot add a boolean");
                self.debug_stack();
                return false;
            }
            ValueType::Array(_) => {
                self.runtime_error("Cannot add an array");
                return false;
            }
        };

        self.push(result);
        true
    }

    pub fn run(&mut self, instructions: &'a Vec<OpCode>) -> bool {
        //dbg!(&instructions);
        if instructions.len() == 0 {
            return true;
        }
        let mut call_frames: Vec<Frame> = Vec::new();
        let main_frame = Frame {
            ip: 0,
            frame_pointer: 0,
        };
        let mut frame = main_frame;
        loop {
            let instr = &instructions[frame.ip];
            self.ip = frame.ip;
            match instr {
                OpCode::ConstantNum(num) => {
                    self.push(ValueType::Number(*num));
                }
                OpCode::ConstantBool(val) => {
                    self.push(ValueType::Boolean(*val));
                }
                OpCode::ConstantStr(str) => {
                    self.push(ValueType::Str(str));
                }
                OpCode::Subtract => {
                    if !self.binary(&instr) {
                        return false;
                    };
                }
                OpCode::Multiply => {
                    if !self.binary(&instr) {
                        return false;
                    };
                }
                OpCode::Mod => {
                    if !self.binary(&instr) {
                        return false;
                    };
                }
                OpCode::Pow => {
                    if !self.binary(&instr) {
                        return false;
                    };
                }
                OpCode::Divide => {
                    if !self.binary(&instr) {
                        return false;
                    };
                }
                OpCode::Add => {
                    if !self.add() {
                        return false;
                    };
                }
                OpCode::Negate => {
                    if !self.negate() {
                        return false;
                    };
                }
                OpCode::Not => {
                    if !self.not() {
                        return false;
                    };
                }
                OpCode::GreaterThan => {
                    if !self.comparison(&instr) {
                        return false;
                    };
                }
                OpCode::GreaterThanEq => {
                    if !self.comparison(&instr) {
                        return false;
                    };
                }
                OpCode::LessThan => {
                    if !self.comparison(&instr) {
                        return false;
                    };
                }
                OpCode::LessThanEq => {
                    if !self.comparison(&instr) {
                        return false;
                    };
                }
                OpCode::Equal => {
                    if !self.comparison(&instr) {
                        return false;
                    };
                }
                OpCode::NotEqual => {
                    if !self.comparison(&instr) {
                        return false;
                    };
                }
                OpCode::SetGlobal(name) => {
                    let v = &self.stack[self.stack_pointer - 1];
                    self.globals.insert(name, v.clone());
                }
                OpCode::GetGlobal(name) => {
                    if let Some(value) = self.globals.get(&name) {
                        self.push(value.to_owned());
                    } else {
                        let message = format!("Global variable {name} does not exist.");
                        self.runtime_error(&message);
                        return false;
                    }
                }
                OpCode::CallNative(index, argc) => {
                    let mut args: Vec<ValueType> = Vec::new();

                    let func = Vm::NATIVES[*index].0;
                    // call a native/built-in function
                    for _i in 0..*argc {
                        // self.stack_pointer -= 1;
                        // let v = &self.stack[self.stack_pointer];
                        pop!(self, v);
                        args.insert(0, v.clone());
                    }
                    let result = func(args, self);

                    match result {
                        Ok(value) => self.push(value),
                        Err(message) => {
                            self.runtime_error(&message);
                            return false;
                        }
                    }
                }
                // OpCode::CallNativeGr(index, argc) => {
                //     let mut args: Vec<ValueType> = Vec::new();
                //     let func = Vm::NATIVES_GR[*index].0;

                //     // call a native/built-in function
                //     for _i in 0..*argc {
                //         pop!(self, v);
                //         args.insert(0, v.clone());
                //     }
                //     if let Err(msg) = func(args, &mut self.gr) {
                //         let message = format!("{}", msg);
                //         self.runtime_error(&message);
                //         return false;
                //     }
                //     self.push(ValueType::Boolean(true));
                // }
                OpCode::CallSystem(name, argc, _) => {
                    let mut args: Vec<ValueType> = Vec::new();
                    for _i in 0..*argc {
                        pop!(self, v);
                        args.insert(0, v.clone());
                    }

                    let result = system_command(name, args);

                    match result {
                        Ok(value) => self.push(value),
                        Err(message) => {
                            self.runtime_error(&message);
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
                OpCode::DefineLocal(i) => {
                    //dbg!(i);
                    let value = self.stack[self.stack_pointer - 1].clone();
                    if i + frame.frame_pointer >= self.stack_pointer - 1 {
                        self.push(value);
                    } else {
                        self.stack[i + frame.frame_pointer] = value;
                    }
                }
                // do a define local
                OpCode::JumpIfFalse(to_jump) => {
                    pop!(self, result);
                    //if let Some(result) = self.stack.pop() {
                    if let ValueType::Boolean(val) = result {
                        if !val {
                            frame.ip += to_jump;
                        }
                    } else {
                        self.runtime_error("boolean value expected");
                        return false;
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
                        // set the call frame
                        frame = value;
                        let val = self.return_value.clone();
                        self.push(val.unwrap());
                    } else {
                        break;
                    }
                }
                OpCode::And | OpCode::Or => {
                    if !self.and_or(instr) {
                        return false;
                    }
                }
                OpCode::Subscript => {
                    pop!(self, index);
                    pop!(self, array);

                    if let ValueType::Array(a) = array {
                        if let ValueType::Number(index) = index {
                            let i = *index as usize;
                            if let Some(val) = a.get(i) {
                                self.push(val.clone());
                            } else {
                                self.runtime_error("Subscript out of range");
                                return false;
                            }
                        } else {
                            self.runtime_error("Subscript index must be a number");
                            return false;
                        }
                    } else {
                        self.runtime_error("Subscript only works on arrays");
                        return false;
                    }
                }
                OpCode::SubscriptSet => {
                    pop!(self, value);
                    pop!(self, index);
                    pop!(self, array);
                    let mut x = array.clone();
                    if let ValueType::Array(ref mut a) = x {
                        if let ValueType::Number(index) = index {
                            let i = *index as usize;
                            a[i] = value.clone();
                            self.push(x);

                            // if let Some(val) = a.get(i) {
                            //     self.push(val.clone());
                            // } else {
                            //     self.runtime_error("Subscript out of range");
                            //     return false;
                            // }
                        } else {
                            self.runtime_error("Subscript index must be a number");
                            return false;
                        }
                    } else {
                        self.runtime_error("Subscript only works on arrays");
                        return false;
                    }
                }
            }
            frame.ip += 1;
            if frame.ip >= instructions.len() {
                break;
            }
            //dbg!(&instr);
            //dbg!(&self.stack[0..self.stack_pointer]);
        }

        true
    }
}
