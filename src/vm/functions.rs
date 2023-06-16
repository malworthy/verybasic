use crate::vm::ValueType;
use std::io::{self, Write};

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
