use crate::vm::ValueType;
use glob::glob;
use hex;
use rand;
use std::{
    fs::read_to_string,
    io::{self, Write},
    time::SystemTime,
};

use super::graphics::Graphics;
use colored::Colorize;

// Parameters: 0(string) = string to print, 1(bool) = print new line if true
pub fn print(params: Vec<ValueType>) -> Result<ValueType, &str> {
    if let Some(val) = params.first() {
        let s = val.to_string();
        let new_line = if let Some(print_new_line) = params.get(1) {
            if let ValueType::Boolean(val) = print_new_line {
                *val
            } else {
                false
            }
        } else {
            true
        };
        let color = match params.get(2) {
            Some(val) => val.to_string(),
            None => String::from("normal"),
        };
        let cs = match color.as_str() {
            "red" => s.red(),
            "green" => s.green(),
            "blue" => s.blue(),
            "yellow" => s.yellow(),
            "white" => s.white(),
            "cyan" => s.cyan(),
            "magenta" => s.magenta(),
            "purple" => s.purple(),
            "black" => s.black(),
            _ => s.normal(),
        };
        if new_line {
            println!("{cs}");
        } else {
            print!("{cs}");
            std::io::stdout().flush().unwrap();
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
        Ok(_) => Result::Ok(ValueType::String(String::from(input.trim_end()))),
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
// String functions
pub fn mid(params: Vec<ValueType>) -> Result<ValueType, &str> {
    if params.len() < 2 {
        return Err(
            "Incorrect number of parameters passed to function mid(string, start[,length])",
        );
    }
    let string = params[0].to_string();
    let start = if let ValueType::Number(val) = params[1] {
        if val < 1.0 {
            return Err("Parameter 'start' of mid(string, start[,length]) must be a 1 or greater");
        }
        val as usize - 1
    } else {
        return Err("Parameter 'start' of mid(string, start[,length]) must be a number");
    };
    if start >= string.len() {
        return Ok(ValueType::String(String::from("")));
    }

    if let Some(param) = params.get(2) {
        let mut length = if let ValueType::Number(val) = param {
            if *val < 0.0 {
                return Err(
                    "Parameter 'length' of mid(string, start[,length]) must be a 0 or greater",
                );
            }
            *val as usize
        } else {
            return Err("Parameter 'start' of mid(string, start[,length]) must be a number");
        };

        if start + length >= string.len() {
            length = string.len() - start;
        }

        let result = &string[start..start + length];
        return Ok(ValueType::String(String::from(result)));
    }

    let result = &string[start..];

    Ok(ValueType::String(String::from(result)))
}

pub fn left(params: Vec<ValueType>) -> Result<ValueType, &str> {
    if params.len() < 2 {
        return Err("Incorrect number of parameters passed to function left(string, length)");
    }
    let string = params[0].to_string();
    let start = if let ValueType::Number(val) = params[1] {
        if val < 0.0 {
            return Err("Parameter 'length' of left(string, length) must be a 0 or greater");
        }
        val as usize
    } else {
        return Err("Parameter 'length' of left(string, length) must be a number");
    };
    if start >= string.len() {
        return Ok(ValueType::String(string));
    }

    Ok(ValueType::String(String::from(&string[..start])))
}

pub fn str(params: Vec<ValueType>) -> Result<ValueType, &str> {
    if params.len() == 0 {
        return Err("Incorrect number of parameters passed to function str(value)");
    }
    let string = params[0].to_string();

    Ok(ValueType::String(string))
}

pub fn floor(params: Vec<ValueType>) -> Result<ValueType, &str> {
    if let Some(param) = params.get(0) {
        if let ValueType::Number(val) = param {
            return Ok(ValueType::Number(val.floor()));
        } else {
            return Err("Parameter passed to function 'floor(num)' must be a number.");
        }
    } else {
        return Err("Incorrect number of parameters passed to function 'floor(num)'");
    }
}

// Graphics functions
pub fn rgb(params: Vec<ValueType>) -> Result<ValueType, &str> {
    let r: u8;
    let g: u8;
    let b: u8;

    if let Some(param) = params.get(0) {
        if let ValueType::Number(val) = param {
            if *val > 255.0 {
                return Err("Red greater than 255");
            }
            r = *val as u8;
        } else {
            return Err("Incorrect parameters passed to function 'rgb'");
        }
    } else {
        return Err("Incorrect parameters passed to function 'rgb'");
    }

    if let Some(param) = params.get(1) {
        if let ValueType::Number(val) = param {
            if *val > 255.0 {
                return Err("Green greater than 255");
            }
            g = *val as u8;
        } else {
            return Err("Incorrect parameters passed to function 'rgb'");
        }
    } else {
        return Err("Incorrect parameters passed to function 'rgb'");
    }

    if let Some(param) = params.get(2) {
        if let ValueType::Number(val) = param {
            if *val > 255.0 {
                return Err("Blue greater than 255");
            }
            b = *val as u8;
        } else {
            return Err("Incorrect parameters passed to function 'rgb'");
        }
    } else {
        return Err("Incorrect parameters passed to function 'rgb'");
    }

    let hex_string = format!("#{}", hex::encode([r, g, b]));
    Ok(ValueType::String(hex_string))
}

pub fn window<'a>(
    _params: Vec<ValueType<'a>>,
    g: &'a mut Graphics,
) -> Result<ValueType<'a>, &'a str> {
    g.show_window();
    Ok(ValueType::Boolean(true))
}

pub fn clear_graphics<'a>(
    _params: Vec<ValueType<'a>>,
    g: &'a mut Graphics,
) -> Result<ValueType<'a>, &'a str> {
    g.clear();
    Ok(ValueType::Boolean(true))
}

pub fn init_graphics<'a>(
    params: Vec<ValueType<'a>>,
    g: &'a mut Graphics,
) -> Result<ValueType<'a>, &'a str> {
    if params.len() < 2 {
        return Err("Not enough parameters passed to function initgraphics(width, height)");
    }
    let width: i32;
    let height: i32;
    if let ValueType::Number(n) = params[0] {
        width = n as i32;
    } else {
        return Err("parameter x must be a number in initgraphics(width, height)");
    }
    if let ValueType::Number(n) = params[1] {
        height = n as i32;
    } else {
        return Err("parameter y must be a number in initgraphics(width, height)");
    }
    g.init(width, height);
    Ok(ValueType::Boolean(true))
}

pub fn plot<'a>(params: Vec<ValueType<'a>>, g: &'a mut Graphics) -> Result<ValueType<'a>, &'a str> {
    if params.len() < 3 {
        return Err("Not enough parameters passed to function plot(x,y,c)");
    }
    let x: f32;
    let y: f32;
    if let ValueType::Number(n) = params[0] {
        x = n as f32;
    } else {
        return Err("parameter x must be a number in plot(x,y,c)");
    }
    if let ValueType::Number(n) = params[1] {
        y = n as f32;
    } else {
        return Err("parameter y must be a number in plot(x,y,c)");
    }

    let colour = params[2].to_string();

    let rgb = match colour.as_str() {
        "darkblue" => (0, 0, 128),
        "blue" => (0, 0, 255),
        "purple" => (128, 0, 128),
        "yellow" => (255, 255, 0),
        "pink" => (255, 192, 203),
        "red" => (255, 0, 0),
        "green" => (0, 255, 0),
        "black" => (0, 0, 0),
        "white" => (255, 255, 255),
        _ => hex_to_rgb(&colour),
    };

    g.draw_rect(x, y, 1.0, 1.0, rgb);

    Ok(ValueType::Boolean(true))
}

fn hex_to_rgb(colour: &str) -> (u8, u8, u8) {
    if !colour.starts_with('#') || colour.len() != 7 {
        return (0, 0, 0);
    }
    let result = hex::decode(&colour[1..]);

    match result {
        Ok(bytes) => (bytes[0], bytes[1], bytes[2]),
        Err(_) => (0, 0, 0),
    }
}

#[cfg(test)]
mod tests {
    use super::hex_to_rgb;

    //use crate::hex_to_rgb;
    #[test]
    fn test_hex_to_rgb() {
        let result = hex_to_rgb("#FF0000");
        assert_eq!(result, (255, 0, 0));

        let result = hex_to_rgb("#0000ff");
        assert_eq!(result, (0, 0, 255));

        let result = hex_to_rgb("#00ff00");
        assert_eq!(result, (0, 255, 0));

        let result = hex_to_rgb("#00ff0");
        assert_eq!(result, (0, 0, 0));

        let result = hex_to_rgb("garbage");
        assert_eq!(result, (0, 0, 0));

        let result = hex_to_rgb("");
        assert_eq!(result, (0, 0, 0));
    }
}
