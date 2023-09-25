use crate::vm::ValueType;
use chrono::{DateTime, Local};
use glob::glob;
use hex;
use rand;
use std::{
    collections::HashMap,
    env,
    fs::{read_to_string, File, OpenOptions},
    io::{self, Write},
    time::SystemTime,
};

use super::Vm;
use colored::Colorize;

// Parameters: 0(string) = string to print, 1(bool) = print new line if true
pub fn print<'a>(params: Vec<ValueType<'a>>, _: &mut Vm<'a>) -> Result<ValueType<'a>, &'a str> {
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
            "black" => s.bright_black(),
            "*red" => s.bright_red(),
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

pub fn input<'a>(params: Vec<ValueType<'a>>, _: &mut Vm<'a>) -> Result<ValueType<'a>, &'a str> {
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

pub fn array<'a>(params: Vec<ValueType<'a>>, _: &mut Vm<'a>) -> Result<ValueType<'a>, &'a str> {
    let mut array: Vec<ValueType> = Vec::new();
    for value in params {
        array.push(value)
    }
    Ok(ValueType::Array(array))
}

pub fn stack<'a>(_params: Vec<ValueType<'a>>, vm: &mut Vm<'a>) -> Result<ValueType<'a>, &'a str> {
    vm.debug_stack();
    Ok(ValueType::Boolean(true))
}

pub fn command<'a>(_params: Vec<ValueType<'a>>, _: &mut Vm<'a>) -> Result<ValueType<'a>, &'a str> {
    let mut array: Vec<ValueType> = Vec::new();
    let args: Vec<String> = env::args().collect();
    for value in args {
        array.push(ValueType::String(value))
    }
    Ok(ValueType::Array(array))
}

pub fn seconds<'a>(_params: Vec<ValueType<'a>>, _: &mut Vm<'a>) -> Result<ValueType<'a>, &'a str> {
    match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
        Ok(n) => Ok(ValueType::Number(n.as_secs_f64())),
        Err(_) => Err("SystemTime before UNIX EPOCH!"),
    }
}

pub fn now<'a>(_params: Vec<ValueType<'a>>, _: &mut Vm<'a>) -> Result<ValueType<'a>, &'a str> {
    let now = SystemTime::now();
    let now: DateTime<Local> = now.into();
    let now = now.to_rfc3339();

    Ok(ValueType::String(now))
}

pub fn len<'a>(params: Vec<ValueType<'a>>, _: &mut Vm<'a>) -> Result<ValueType<'a>, &'a str> {
    if let Some(val) = params.first() {
        let len = match val {
            ValueType::Array(v) => v.len(),
            ValueType::Str(s) => s.len(),
            ValueType::String(s) => s.len(),
            ValueType::Number(_) => 8,
            ValueType::Boolean(_) => 1,
            //ValueType::Struct(s) => s.len(),
            //ValueType::PointerG(_) => panic!("Pointers not implemented"),
            //ValueType::PointerL(_) => panic!("Pointers not implemented"),
        };
        let len = len as f64;
        Ok(ValueType::Number(len))
    } else {
        Err("No parameters passed to function len()")
    }
}

pub fn dir<'a>(params: Vec<ValueType<'a>>, _: &mut Vm<'a>) -> Result<ValueType<'a>, &'a str> {
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

pub fn random<'a>(_params: Vec<ValueType<'a>>, _: &mut Vm<'a>) -> Result<ValueType<'a>, &'a str> {
    let number = rand::random::<f64>();
    Ok(ValueType::Number(number))
}

pub fn compare(a: &ValueType, b: &ValueType) -> std::cmp::Ordering {
    let a = match a {
        ValueType::Number(n) => n,
        _ => &0.0,
    };
    let b = match b {
        ValueType::Number(n) => n,
        _ => &0.0,
    };
    a.partial_cmp(&b).unwrap()
}

pub fn sort<'a>(params: Vec<ValueType<'a>>, _: &mut Vm<'a>) -> Result<ValueType<'a>, &'a str> {
    if let Some(param) = params.first() {
        if let ValueType::Array(vec) = param {
            let mut result = vec.clone();
            result.sort_by(|a, b| compare(a, b));
            return Ok(ValueType::Array(result));
        }
    }
    Err("Incorrect parameters passed to sort(array)")
}

pub fn push_mut<'a>(
    array: &mut ValueType<'a>,
    params: Vec<ValueType<'a>>,
) -> Result<ValueType<'a>, &'a str> {
    if params.len() < 1 {
        return Err("Incorrect parameters passed to push(array, value)");
    }
    let value = params.iter().next();
    if let Some(val) = value {
        //let array = &mut params[0];
        //let v = val.clone();
        if let ValueType::Array(ref mut vec) = array {
            vec.push(val.clone());
            //let mut result = vec.clone();
            //result.push(value);
            return Ok(ValueType::Boolean(true));
            //return Ok(ValueType::Array(vec.clone()));
            //return Ok(val.to_owned());
        }
    }

    Err("Incorrect parameters passed to  push(array, value)")
}

pub fn slice<'a>(
    array: &mut ValueType<'a>,
    params: Vec<ValueType<'a>>,
) -> Result<ValueType<'a>, &'a str> {
    if params.len() < 2 {
        return Err("Incorrect number of parameters passed to slice(start, finish)");
    }
    let start = params.iter().nth(0).unwrap();
    let finish = params.iter().nth(1).unwrap();
    let start = if let ValueType::Number(n) = start {
        n
    } else {
        return Err("Incorrect parameters passed to  slice(start, finish)");
    };
    let finish = if let ValueType::Number(n) = finish {
        n
    } else {
        return Err("Incorrect parameters passed to slice(start, finish)");
    };
    if *start < 0.0 || *finish < 0.0 {
        return Err("Incorrect parameters passed to slice(start, finish). Start and finish must be greater than zero.");
    }
    dbg!(&array);
    if let ValueType::Array(ref mut vec) = array {
        dbg!(&vec);
        let sliced = &vec[*start as usize..*finish as usize];
        dbg!(sliced);
        return Ok(ValueType::Array(sliced.to_owned()));
    } else {
        return Err("Incorrect parameters passed to slice(start, finish)");
    };
}

pub fn push<'a>(params: Vec<ValueType<'a>>, _: &mut Vm<'a>) -> Result<ValueType<'a>, &'a str> {
    if params.len() < 2 {
        return Err("Incorrect parameters passed to push(array, value)");
    }
    let array = &params[0];
    let value = params[1].clone();
    if let ValueType::Array(vec) = array {
        let mut result = vec.clone();
        result.push(value);
        return Ok(ValueType::Array(result));
    }
    Err("Incorrect parameters passed to  push(array, value)")
}

pub fn chr<'a>(params: Vec<ValueType<'a>>, _: &mut Vm<'a>) -> Result<ValueType<'a>, &'a str> {
    if let Some(param) = params.first() {
        if let ValueType::Number(num) = param {
            if *num >= 0.0 && *num <= 255.0 {
                let num = *num as u8;
                if num.is_ascii() {
                    let ch = num as char;
                    return Ok(ValueType::String(ch.to_string()));
                }
            }
        }
    }
    Ok(ValueType::Str(""))
}

pub fn readlines<'a>(params: Vec<ValueType<'a>>, _: &mut Vm<'a>) -> Result<ValueType<'a>, &'a str> {
    if let Some(param) = params.first() {
        let filename = param.to_string();

        let result = read_to_string(filename);
        let lines: Vec<ValueType> = match result {
            Ok(text) => text
                .lines()
                .map(|x| ValueType::String(String::from(x)))
                .collect(),
            Err(_) => ""
                .lines()
                .map(|x| ValueType::String(String::from(x)))
                .collect(),
        };
        Ok(ValueType::Array(lines))
    } else {
        Err("No parameters passed to readlines()")
    }
}

pub fn setting_set<'a>(
    params: Vec<ValueType<'a>>,
    vm: &mut Vm<'a>,
) -> Result<ValueType<'a>, &'a str> {
    if params.len() < 2 {
        return Err("Incorrect number of parameters passed to setting_set(key, value)");
    }

    let result = read_to_string(&vm.config_file);
    let mut settings: HashMap<&str, String> = HashMap::new();

    let file_contents = match result {
        Ok(lines) => lines,
        Err(_) => String::new(),
    };
    if !file_contents.is_empty() {
        settings = serde_json::from_str(&file_contents).unwrap();
    }

    let key = params[0].to_string();
    let value = params[1].to_string();

    settings.insert(key.as_str(), value);

    let json = serde_json::to_string(&settings).unwrap();
    let data_file = File::create(&vm.config_file);

    match data_file {
        Ok(mut file) => {
            let result = file.write(json.as_bytes());
            return match result {
                Ok(_) => Ok(ValueType::Boolean(true)),
                Err(msg) => Ok(ValueType::String(msg.to_string())),
            };
        }
        Err(msg) => {
            return Ok(ValueType::String(msg.to_string()));
        }
    }
}

pub fn setting_get<'a>(
    params: Vec<ValueType<'a>>,
    vm: &mut Vm<'a>,
) -> Result<ValueType<'a>, &'a str> {
    if params.len() < 1 {
        return Err("Incorrect number of parameters passed to setting_get(key)");
    }

    let result = read_to_string(&vm.config_file);
    let settings: HashMap<&str, String>;

    let file_contents = match result {
        Ok(lines) => lines,
        Err(_) => return Ok(ValueType::Str("")),
    };
    settings = serde_json::from_str(&file_contents).unwrap();
    let key = params[0].to_string();

    let value = if let Some(value) = settings.get(&key.as_str()) {
        value
    } else {
        ""
    };

    Ok(ValueType::String(value.to_string()))
}

pub fn write<'a>(params: Vec<ValueType<'a>>, _: &mut Vm<'a>) -> Result<ValueType<'a>, &'a str> {
    let mut params_iter = params.iter();
    let p1 = params_iter.next();
    let p2 = params_iter.next();
    if let None = p2 {
        return Err("Missing parameter passed to append(filename, text_to_write)");
    }
    if let Some(param) = p1 {
        let filename = param.to_string();
        let contents = p2.unwrap().to_string();

        let data_file = File::create(filename);

        match data_file {
            Ok(mut file) => {
                let result = file.write(contents.as_bytes());
                return match result {
                    Ok(_) => Ok(ValueType::Boolean(true)),
                    Err(msg) => Ok(ValueType::String(msg.to_string())),
                };
            }
            Err(msg) => {
                return Ok(ValueType::String(msg.to_string()));
            }
        }
    } else {
        Err("No parameters passed to append(filename, text_to_write)")
    }
}

pub fn append<'a>(params: Vec<ValueType<'a>>, _: &mut Vm<'a>) -> Result<ValueType<'a>, &'a str> {
    let mut params_iter = params.iter();
    let p1 = params_iter.next();
    let p2 = params_iter.next();
    if let None = p2 {
        return Err("Missing parameter passed to append(filename, text_to_write)");
    }
    if let Some(param) = p1 {
        let filename = param.to_string();
        let contents = p2.unwrap().to_string();

        let data_file = OpenOptions::new().append(true).create(true).open(filename);

        match data_file {
            Ok(mut file) => {
                let result = file.write(contents.as_bytes());
                return match result {
                    Ok(_) => Ok(ValueType::Boolean(true)),
                    Err(msg) => Ok(ValueType::String(msg.to_string())),
                };
            }
            Err(msg) => {
                return Ok(ValueType::String(msg.to_string()));
            }
        }
    } else {
        Err("No parameters passed to append(filename, text_to_write)")
    }
}

pub fn val<'a>(params: Vec<ValueType<'a>>, _: &mut Vm<'a>) -> Result<ValueType<'a>, &'a str> {
    if params.len() == 0 {
        return Err("Incorrect number of parameters passed to function str(value)");
    }
    let parsed = params[0].to_string().parse::<f64>();

    match parsed {
        Ok(num) => Ok(ValueType::Number(num)),
        Err(_) => Ok(ValueType::Number(0.0)),
    }
}

pub fn floor<'a>(params: Vec<ValueType<'a>>, _: &mut Vm<'a>) -> Result<ValueType<'a>, &'a str> {
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

pub fn sqrt<'a>(params: Vec<ValueType<'a>>, _: &mut Vm<'a>) -> Result<ValueType<'a>, &'a str> {
    if let Some(param) = params.get(0) {
        if let ValueType::Number(val) = param {
            return Ok(ValueType::Number(val.sqrt()));
        } else {
            return Err("Parameter passed to function 'floor(num)' must be a number.");
        }
    } else {
        return Err("Incorrect number of parameters passed to function 'floor(num)'");
    }
}

// Graphics functions
pub fn rgb<'a>(params: Vec<ValueType<'a>>, _: &mut Vm<'a>) -> Result<ValueType<'a>, &'a str> {
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

pub fn window<'a>(_params: Vec<ValueType<'a>>, vm: &mut Vm<'a>) -> Result<ValueType<'a>, &'a str> {
    vm.gr.show_window();
    Ok(ValueType::Boolean(true))
}

pub fn clear_graphics<'a>(
    _params: Vec<ValueType<'a>>,
    vm: &mut Vm<'a>,
) -> Result<ValueType<'a>, &'a str> {
    vm.gr.clear();
    Ok(ValueType::Boolean(true))
}

pub fn init_graphics<'a>(
    params: Vec<ValueType<'a>>,
    vm: &mut Vm<'a>,
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
    vm.gr.init(width, height);
    Ok(ValueType::Boolean(true))
}

pub fn plot<'a>(params: Vec<ValueType<'a>>, vm: &mut Vm<'a>) -> Result<ValueType<'a>, &'a str> {
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

    vm.gr.draw_rect(x, y, 1.0, 1.0, rgb);

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
