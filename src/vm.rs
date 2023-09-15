mod functions;
mod graphics;
mod string_functions;
use std::{
    collections::HashMap,
    io::{self, Write},
    path::PathBuf,
    process::Command,
};

use crate::compiler::OpCode;
use colored::Colorize;

#[derive(Debug, Clone)]
pub enum ValueType<'a> {
    Number(f64),
    Str(&'a str),
    Boolean(bool),
    String(String),
    Array(Vec<ValueType<'a>>),
    Struct(HashMap<&'a str, ValueType<'a>>),
    PointerG(u32),
    PointerL(usize),
}

impl ValueType<'_> {
    pub fn to_string(&self) -> String {
        match self {
            ValueType::Number(n) => format!("{n}"),
            ValueType::Boolean(b) => format!("{b}"),
            ValueType::Str(str) => str.to_string(),
            ValueType::String(str) => str.to_string(),
            ValueType::Array(a) => format!("{:?}", a),
            ValueType::Struct(s) => format!("{:?}", s),
            ValueType::PointerG(_) => panic!("Pointers not implemented"),
            ValueType::PointerL(_) => panic!("Pointers not implemented"),
        }
    }
}

#[derive(Copy, Clone, Debug)]
struct Frame {
    ip: usize,
    frame_pointer: usize,
    //mut_call: bool,
    // local: usize,
    // global: &'a str,
}

const MAX_STACK: usize = 128;

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

macro_rules! pop2 {
    ($s:ident, $v:ident) => {
        $s.stack_pointer -= 1;
        let $v = &$s.stack[$s.stack_pointer];
    };
}

macro_rules! pop {
    ($s:ident, $v:ident) => {
        $s.stack_pointer -= 1;
        let $v = match &$s.stack[$s.stack_pointer] {
            ValueType::PointerG(p) => $s.globals.get(p).unwrap(),
            ValueType::PointerL(p) => &$s.stack[*p],
            _ => &$s.stack[$s.stack_pointer],
        };
        // Need to handle more than 1 level of pointers!
        let $v = match $v {
            ValueType::PointerG(p) => $s.globals.get(p).unwrap(),
            ValueType::PointerL(p) => &$s.stack[*p],
            _ => $v,
        };
    };
}

macro_rules! peek {
    ($s:ident, $v:ident, $index:ident) => {
        let $v = match &$s.stack[$index] {
            ValueType::PointerG(p) => $s.globals.get_mut(p).unwrap(),
            ValueType::PointerL(p) => &mut $s.stack[*p],
            _ => &mut $s.stack[$index],
        };

        // let mut $v = match $v {
        //     ValueType::PointerG(p) => $s.globals.get_mut(p).unwrap(),
        //     ValueType::PointerL(p) => &mut $s.stack[*p],
        //     _ => $v,
        // };
    };
}

pub struct DebugSettings {
    pub break_points: Vec<u32>,
    pub code_window: u32,
}

impl DebugSettings {
    pub fn new(code_window: u32, break_points: &str) -> Self {
        let iter = break_points.split(',');
        let mut break_points: Vec<u32> = Vec::new();
        for i in iter {
            let parsed = i.parse::<u32>();
            if let Ok(line_num) = parsed {
                break_points.push(line_num);
            }
        }
        DebugSettings {
            break_points,
            code_window,
        }
    }
}

enum DebugStep {
    Continue,
    StepInto,
    StepOver,
}

pub struct Vm<'a> {
    stack: [ValueType<'a>; MAX_STACK],
    //value_pointers: [ValuePointer; MAX_STACK],
    stack_pointer: usize,
    globals: HashMap<u32, ValueType<'a>>,
    pub return_value: Option<ValueType<'a>>,
    gr: graphics::Graphics,
    line_numbers: &'a mut Vec<u32>,
    ip: usize,
    pub config_file: PathBuf,
    in_error: bool,
    // for debugging
    source_code: Option<&'a Vec<&'a str>>,
    debug_settings: Option<DebugSettings>,
    step: DebugStep,
    break_frame: usize,
}

impl<'a> Vm<'a> {
    pub fn new(line_numbers: &'a mut Vec<u32>) -> Self {
        Vm {
            //stack: Vec::new(),
            stack: [EMPTY_ELEMENT; MAX_STACK],
            //value_pointers: [NO_POINTER; MAX_STACK],
            globals: HashMap::new(),
            return_value: Option::None,
            gr: graphics::Graphics::new(),
            stack_pointer: 0,
            ip: 0,
            line_numbers,
            config_file: PathBuf::from("settings.txt"),
            in_error: false,
            source_code: None,
            debug_settings: None,
            step: DebugStep::Continue,
            break_frame: 0,
        }
    }

    pub fn new_debug(
        line_numbers: &'a mut Vec<u32>,
        source_code: &'a Vec<&'a str>,
        settings: DebugSettings,
    ) -> Self {
        Vm {
            stack: [EMPTY_ELEMENT; MAX_STACK],
            //value_pointers: [NO_POINTER; MAX_STACK],
            globals: HashMap::new(),
            return_value: Option::None,
            gr: graphics::Graphics::new(),
            stack_pointer: 0,
            ip: 0,
            line_numbers,
            config_file: PathBuf::from("settings.txt"),
            in_error: false,
            source_code: Some(source_code),
            debug_settings: Some(settings),
            step: DebugStep::Continue,
            break_frame: 0,
        }
    }

    pub const MUT_NATIVES: [(
        fn(array: &mut ValueType<'a>, params: Vec<ValueType<'a>>) -> Result<ValueType<'a>, &'a str>,
        &str,
    ); 1] = [(functions::push_mut, "push")];

    pub const NATIVES: [(
        fn(Vec<ValueType<'a>>, &mut Vm<'a>) -> Result<ValueType<'a>, &'a str>,
        &str,
    ); 33] = [
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
        (functions::sqrt, "sqrt"),
    ];

    pub fn debug_stack(&mut self) {
        dbg!(&self.stack[0..self.stack_pointer + 1]);
    }

    fn runtime_error(&mut self, message: &str) {
        let line_number = self.line_numbers[self.ip];
        eprintln!("Runtime error: {} in line {line_number}", message.red());
        self.in_error = true;
    }

    fn push(&mut self, value: ValueType<'a>) {
        if self.stack_pointer >= MAX_STACK {
            self.runtime_error("Stack Overflow");
            return;
            //panic!("Stack overflow. TODO: Remove this panic and handle the error gracefully!")
        }
        self.stack[self.stack_pointer] = value;
        //self.value_pointers[self.stack_pointer] = ValuePointer::None;
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
            ValueType::Boolean(a) => match b {
                ValueType::Boolean(b) => ValueType::Boolean(a == b),
                _ => {
                    self.runtime_error("Cannot compare a boolean to a non boolean type.");
                    return false;
                }
            },
            ValueType::Array(_) => {
                self.runtime_error("Array not valid for comparison operation");
                return false;
            }
            ValueType::Struct(_) => {
                self.runtime_error("Data structure not valid for comparison operation");
                return false;
            }
            ValueType::PointerG(_) => panic!("Pointers not implemented"),
            ValueType::PointerL(_) => panic!("Pointers not implemented"),
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
                self.runtime_error("'not' invalid for an array");
                return false;
            }
            ValueType::Struct(_) => {
                self.runtime_error("'not' invalid for an data structure");
                return false;
            }
            ValueType::PointerG(_) => panic!("Pointers not implemented"),
            ValueType::PointerL(_) => panic!("Pointers not implemented"),
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
            ValueType::Struct(_) => {
                self.runtime_error("Cannot add a data structure");
                return false;
            }
            ValueType::PointerG(_) => panic!("Pointers not implemented"),
            ValueType::PointerL(_) => panic!("Pointers not implemented"),
        };

        self.push(result);
        true
    }

    fn debug(&mut self, ip: usize, frame: &Frame, frame_index: usize) {
        let settings = self.debug_settings.as_ref().unwrap();

        let mut break_line: u32 = 0;
        let source_lines = self.source_code.unwrap();
        let step_into = match self.step {
            DebugStep::StepOver => {
                if self.line_numbers[ip] > 0 {
                    break_line = self.line_numbers[ip];
                }
                false
            }
            DebugStep::Continue => {
                if settings.break_points.contains(&self.line_numbers[ip]) {
                    break_line = self.line_numbers[ip];
                    self.break_frame = frame_index;
                }
                false
            }
            DebugStep::StepInto => {
                if self.line_numbers[ip] > 0 {
                    break_line = self.line_numbers[ip];
                }
                true
            }
        };

        if self.line_numbers[ip] == break_line
            && break_line > 0
            && (ip == 0 || self.line_numbers[ip - 1] != break_line)
            && (frame_index <= self.break_frame || step_into)
        {
            self.break_frame = frame_index;
            let code_window = self.debug_settings.as_ref().unwrap().code_window;
            let start = if break_line > code_window {
                break_line - code_window
            } else {
                0
            };
            let end = if source_lines.len() > code_window as usize
                && break_line < source_lines.len() as u32 - code_window
            {
                break_line + code_window
            } else {
                source_lines.len() as u32
            };

            println!("<--------CODE------------>");
            for i in start..end {
                if (i + 1) == break_line {
                    println!(">{:5.0} {}", i + 1, source_lines[i as usize].yellow());
                } else {
                    println!("{:6.0} {}", i + 1, source_lines[i as usize]);
                }
            }

            println!("<---- Stack (Locals) --->");
            println!("{:?}", &self.stack[frame.frame_pointer..self.stack_pointer]);

            //println!("<------- Globals ------->");
            //println!("{:?}", &self.globals);
            //println!("<----------------------->");
            let mut input = String::new();
            print!("(S)tep over, step (I)nto, (C)ontinue: ");
            std::io::stdout().flush().unwrap();
            io::stdin().read_line(&mut input).unwrap();
            let input = input.trim();
            match input {
                "i" => self.step = DebugStep::StepInto,
                "c" => self.step = DebugStep::Continue,
                _ => self.step = DebugStep::StepOver,
            }
        }
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
            //mut_call: false,
            // local: 0,
            // global: "",
        };
        let mut frame = main_frame;
        loop {
            if let Some(_) = self.debug_settings {
                self.debug(frame.ip, &frame, call_frames.len());
            }
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
                    //pop!(self, v);
                    self.globals.insert(*name, v.clone());
                    //self.stack_pointer += 1;
                }
                OpCode::GetGlobal(name, use_pointer) => {
                    //let value = self.globals.get_mut(name).unwrap();
                    if *use_pointer {
                        self.push(ValueType::PointerG(*name));
                    } else {
                        if let Some(value) = self.globals.get(name) {
                            //dbg!(&value);
                            self.push(value.to_owned());
                        } else {
                            let message = format!("Global variable {name} does not exist.");
                            self.runtime_error(&message);
                            return false;
                        }
                    }
                    //dbg!(&self.stack[0..self.stack_pointer]);
                }
                OpCode::CallNativeMut(index, argc) => {
                    let mut args: Vec<ValueType> = Vec::new();
                    //dbg!(&self.stack[0..self.stack_pointer]);
                    let func = Vm::MUT_NATIVES[*index].0;
                    for _i in 0..*argc {
                        pop!(self, v);
                        args.insert(0, v.clone());
                    }
                    //pop_pointer!(self, p);
                    pop2!(self, p);

                    let result = match p {
                        ValueType::PointerL(index) => func(&mut self.stack[*index], args),
                        ValueType::PointerG(index) => {
                            func(self.globals.get_mut(index).unwrap(), args)
                        }
                        _ => func(&mut self.stack[self.stack_pointer - 1], args),
                    };

                    match result {
                        Ok(value) => self.push(value),
                        Err(message) => {
                            self.runtime_error(&message);
                            return false;
                        }
                    }
                }
                OpCode::CallNative(index, argc) => {
                    let mut args: Vec<ValueType> = Vec::new();

                    let func = Vm::NATIVES[*index].0;
                    // call a native/built-in function
                    for _i in 0..*argc {
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
                        //mut_call: false,
                        // local: 0,
                        // global: "",
                    };
                }
                OpCode::CallMut(pointer, argc) => {
                    //dbg!(argc);
                    call_frames.push(frame); // save current frame
                    let argc = *argc as usize;
                    frame = Frame {
                        ip: pointer - 1,
                        frame_pointer: self.stack_pointer - argc - 1,
                        //mut_call: true,
                    };
                }
                OpCode::Pop => {
                    pop!(self, v);
                    self.return_value = Some(v.clone());
                }
                OpCode::Pop2 => {
                    pop!(self, _v);
                }
                OpCode::GetLocal(i, use_pointer) => {
                    if *use_pointer {
                        self.push(ValueType::PointerL(*i + frame.frame_pointer))
                    } else {
                        self.push(self.stack[i + frame.frame_pointer].clone());
                    }
                }
                OpCode::SetLocal(i) => {
                    let value = self.stack[self.stack_pointer - 1].clone();
                    match self.stack[i + frame.frame_pointer] {
                        ValueType::PointerG(p) => {
                            self.globals.insert(p, value);
                        }
                        ValueType::PointerL(p) => {
                            self.stack[p] = value;
                        }
                        _ => self.stack[i + frame.frame_pointer] = value,
                    };
                    //self.stack[i + frame.frame_pointer] = value;
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
                    //dbg!(&self.stack[0..self.stack_pointer]);
                    //dbg!(&self.value_pointers[0..self.stack_pointer]);
                    // pop the frame
                    // if no frames left, then break
                    if let Some(value) = call_frames.pop() {
                        // get rid of any local variables on the stack
                        self.stack_pointer = frame.frame_pointer;
                        //dbg!(&self.stack[self.stack_pointer]);
                        // if frame.mut_call {
                        //     match &self.stack[self.stack_pointer] {
                        //         ValueType::PointerG(g) => {
                        //             let val = &self.stack[self.stack_pointer];
                        //             self.globals.insert(*g, val.clone());
                        //         }
                        //         ValueType::PointerL(index) => {
                        //             let val = self.stack[self.stack_pointer].clone();
                        //             self.stack[index + value.frame_pointer] = val;
                        //         }
                        //         _ => {}
                        //     }
                        // }

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
                OpCode::NewStruct => {
                    let value = ValueType::Struct(HashMap::new());
                    self.push(value);
                }
                OpCode::GetProp(prop) => {
                    pop!(self, value);

                    if let ValueType::Struct(hash_map) = value {
                        let result = hash_map.get(prop.as_str());
                        match result {
                            Some(v) => self.push(v.clone()),
                            None => {
                                let message = format!("Property {} not found", prop);
                                self.runtime_error(&message);
                                return false;
                            }
                        }
                    } else {
                        dbg!(value);
                        self.runtime_error("data type expected")
                    }
                }
                OpCode::AddData(key) => {
                    pop!(self, value);
                    let v = match value {
                        ValueType::Str(x) => ValueType::Str(x),
                        ValueType::Array(x) => ValueType::Array(x.to_vec()),
                        ValueType::Boolean(x) => ValueType::Boolean(*x),
                        ValueType::Number(x) => ValueType::Number(*x),
                        ValueType::String(x) => ValueType::String(x.to_string()),
                        ValueType::Struct(x) => ValueType::Struct(x.clone()),
                        ValueType::PointerG(_) => panic!("Pointers not implemented"),
                        ValueType::PointerL(_) => panic!("Pointers not implemented"),
                    };

                    if let ValueType::Struct(ref mut hash_map) = self.stack[self.stack_pointer - 1]
                    {
                        hash_map.insert(key, v);
                    } else {
                        panic!("Expect a HashMap on top of the stack -  programmer did the wrong thing in the compiler code");
                    }
                }
                OpCode::Subscript => {
                    //dbg!(&self.stack[0..self.stack_pointer]);
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
                        dbg!(&array);
                        dbg!(index);
                        self.runtime_error("Subscript only works on arrays");
                        return false;
                    }
                }
                OpCode::SetProp(name) => {
                    //dbg!(&self.stack[0..self.stack_pointer]);
                    //dbg!(&self.value_pointers[0..self.stack_pointer]);

                    pop!(self, value);
                    //dbg!(value);
                    let value = match value {
                        ValueType::Str(x) => ValueType::Str(x),
                        ValueType::Array(x) => ValueType::Array(x.to_vec()),
                        ValueType::Boolean(x) => ValueType::Boolean(*x),
                        ValueType::Number(x) => ValueType::Number(*x),
                        ValueType::String(x) => ValueType::String(x.to_string()),
                        ValueType::Struct(x) => ValueType::Struct(x.clone()),
                        ValueType::PointerG(_) => panic!("Pointers not implemented"),
                        ValueType::PointerL(_) => panic!("Pointers not implemented"),
                    };

                    pop2!(self, data_pointer);
                    //dbg!(data_pointer);
                    match data_pointer {
                        ValueType::PointerL(i) => {
                            if let ValueType::Struct(ref mut a) = self.stack[*i] {
                                a.insert(&name, value.clone());
                                self.push(value);
                            } else {
                                self.runtime_error("Subscript only works on arrays");
                                return false;
                            }
                        }
                        ValueType::PointerG(s) => {
                            let array = self.globals.get_mut(s).unwrap();
                            if let ValueType::Struct(ref mut a) = array {
                                a.insert(&name, value.clone());
                                self.push(value);
                            } else {
                                self.runtime_error("Subscript only works on arrays");
                                return false;
                            }
                        }
                        _ => {
                            self.runtime_error("Invalid use of subscript");
                            return false;
                        }
                    }
                }

                OpCode::SubscriptSet => {
                    let mut stack = self.stack.iter();
                    let index = stack.nth(self.stack_pointer - 2).unwrap();
                    let value = stack.nth(0).unwrap();

                    self.stack_pointer -= 2;

                    let index = match index {
                        ValueType::Number(n) => *n as usize,
                        _ => {
                            self.runtime_error("Subscript index must be a number");
                            return false;
                        }
                    };
                    let value = match value {
                        ValueType::Str(x) => ValueType::Str(x),
                        ValueType::Array(x) => ValueType::Array(x.to_vec()),
                        ValueType::Boolean(x) => ValueType::Boolean(*x),
                        ValueType::Number(x) => ValueType::Number(*x),
                        ValueType::String(x) => ValueType::String(x.to_string()),
                        ValueType::Struct(x) => ValueType::Struct(x.clone()),
                        ValueType::PointerG(_) => panic!("Pointers not implemented"),
                        ValueType::PointerL(_) => panic!("Pointers not implemented"),
                    };
                    //pop_pointer!(self, array_pointer);
                    pop2!(self, array_pointer);
                    match array_pointer {
                        ValueType::PointerL(i) => {
                            let i = *i;
                            peek!(self, array_value, i);
                            if let ValueType::Array(ref mut a) = array_value {
                                // self.stack[*i] {
                                a[index] = value.clone();
                                self.push(value);
                            } else {
                                self.runtime_error("Subscript only works on arrays");
                                return false;
                            }
                        }
                        ValueType::PointerG(s) => {
                            let array = self.globals.get_mut(s).unwrap();
                            if let ValueType::Array(ref mut a) = array {
                                a[index] = value.clone();
                                self.push(value);
                            } else {
                                self.runtime_error("Subscript only works on arrays");
                                return false;
                            }
                        }
                        _ => {
                            self.runtime_error("Invalid use of subscript");
                            return false;
                        }
                    }
                }
            }
            frame.ip += 1;
            if frame.ip >= instructions.len() {
                break;
            }
            if self.in_error {
                return false;
            }
            //dbg!(&instr);
            //dbg!(&self.stack[0..self.stack_pointer]);
        }

        true
    }
}
