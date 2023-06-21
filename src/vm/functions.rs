use crate::vm::ValueType;
use glob::glob;
use rand;
use std::{
    fs::read_to_string,
    io::{self, Write},
    time::SystemTime,
};

// Parameters: 0(string) = string to print, 1(bool) = print new line if true
pub fn print(params: Vec<ValueType>) -> Result<ValueType, &str> {
    if let Some(val) = params.first() {
        let s = val.to_string();
        if let Some(print_new_line) = params.get(1) {
            //println!("I'm here!");
            print!("{s}");
            std::io::stdout().flush().unwrap();
        } else {
            println!("{s}");
        }
        Result::Ok(ValueType::String(s))
    } else {
        Err("No parameters passed to function")
    }
}

pub fn input(params: Vec<ValueType>) -> Result<ValueType, &str> {
    if let Some(val) = params.first() {
        let s = val.to_string();
        print!("{s} ");
        std::io::stdout().flush().unwrap();
    }
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => Result::Ok(ValueType::String(input)),
        Err(_) => Err("Could not read from terminal"),
    }
}

pub fn array(params: Vec<ValueType>) -> Result<ValueType, &str> {
    let mut array: Vec<ValueType> = Vec::new();
    for value in params {
        array.push(value)
    }
    Ok(ValueType::Array(array))
}

pub fn seconds(_params: Vec<ValueType>) -> Result<ValueType, &str> {
    match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
        Ok(n) => Ok(ValueType::Number(n.as_secs_f64())),
        Err(_) => Err("SystemTime before UNIX EPOCH!"),
    }
}

pub fn len(params: Vec<ValueType>) -> Result<ValueType, &str> {
    if let Some(val) = params.first() {
        let len = match val {
            ValueType::Array(v) => v.len(),
            ValueType::Str(s) => s.len(),
            ValueType::String(s) => s.len(),
            ValueType::Number(_) => 8,
            ValueType::Boolean(_) => 1,
        };
        let len = len as f64;
        Ok(ValueType::Number(len))
    } else {
        Err("No parameters passed to function len()")
    }
}

pub fn dir(params: Vec<ValueType>) -> Result<ValueType, &str> {
    if let Some(val) = params.first() {
        let pattern = val.to_string();
        let file = glob(pattern.as_str());
        match file {
            Ok(paths) => {
                let mut array: Vec<ValueType> = Vec::new();
                for file in paths {
                    if let Ok(file) = file {
                        let s = file.to_string_lossy();
                        let y = String::from(s);
                        array.push(ValueType::String(y));
                    }
                }
                return Ok(ValueType::Array(array));
            }
            Err(_) => {
                return Err("can't read");
            }
        }
    } else {
        Err("Missing parameter for dir()")
    }
}

pub fn random(_params: Vec<ValueType>) -> Result<ValueType, &str> {
    let number = rand::random::<f64>();
    Ok(ValueType::Number(number))
}

pub fn readlines(params: Vec<ValueType>) -> Result<ValueType, &str> {
    if let Some(param) = params.first() {
        let filename = param.to_string();

        let lines: Vec<ValueType> = read_to_string(filename)
            .unwrap() // panic on possible file-reading errors
            .lines() // split the string into an iterator of string slices
            .map(|x| ValueType::String(String::from(x)))
            .collect(); // gather them together into a vector

        Ok(ValueType::Array(lines))
    } else {
        Err("No parameters passed to readlines()")
    }
}
