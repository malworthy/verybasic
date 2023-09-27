use crate::vm::ValueType;
use num_runtime_fmt::NumFmt;

use super::Vm;

// String functions
pub fn mid<'a>(params: Vec<ValueType<'a>>, _: &mut Vm<'a>) -> Result<ValueType<'a>, &'a str> {
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

pub fn left<'a>(params: Vec<ValueType<'a>>, _: &mut Vm<'a>) -> Result<ValueType<'a>, &'a str> {
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

pub fn right<'a>(params: Vec<ValueType<'a>>, _: &mut Vm<'a>) -> Result<ValueType<'a>, &'a str> {
    if params.len() < 2 {
        return Err("Incorrect number of parameters passed to function right(string, length)");
    }
    let string = params[0].to_string();
    let length = if let ValueType::Number(val) = params[1] {
        if val < 0.0 {
            return Err("Parameter 'length' of right(string, length) must be a 0 or greater");
        }
        val as usize
    } else {
        return Err("Parameter 'length' of right(string, length) must be a number");
    };
    if length >= string.len() {
        return Ok(ValueType::String(string));
    }
    let start = string.len() - length;
    Ok(ValueType::String(String::from(&string[start..])))
}

// really rust - no library function to do this???
fn round(number: f64, decimal_places: u32) -> f64 {
    let y = 10i32.pow(decimal_places) as f64;
    (number * y).round() / y
}

pub fn vb_format_num(format: String, number: f64) -> (String, f64) {
    //N
    if format.starts_with("N") {
        let precision = if format.len() == 1 { "2" } else { &format[1..] };
        if let Ok(prec) = precision.parse::<u32>() {
            return (format!("{}.{prec},", prec + 2), round(number, prec));
        }
    }

    //F
    if format.starts_with("F") {
        let precision = if format.len() == 1 { "2" } else { &format[1..] };
        if let Ok(prec) = precision.parse::<u32>() {
            return (format!("{}.{prec}", prec + 2), round(number, prec));
        }
    }

    return (format, number);
}

pub fn str<'a>(params: Vec<ValueType<'a>>, _: &mut Vm<'a>) -> Result<ValueType<'a>, &'a str> {
    if params.len() == 0 {
        return Err("Incorrect number of parameters passed to function str(value)");
    }

    let mut string = params[0].to_string();
    if let Some(format) = params.iter().nth(1) {
        if let ValueType::Number(number) = params[0] {
            let (format_string, number) = vb_format_num(format.to_string(), number);
            if let Ok(formatter) = NumFmt::from_str(format_string.as_str()) {
                if let Ok(formatted) = formatter.fmt(number) {
                    string = formatted;
                    if string.ends_with('.') {
                        string = string[..string.len() - 1].to_string();
                    }
                }
            }
        }
    }

    Ok(ValueType::String(string))
}

pub fn lcase<'a>(params: Vec<ValueType<'a>>, _: &mut Vm<'a>) -> Result<ValueType<'a>, &'a str> {
    if params.len() == 0 {
        return Err("Incorrect number of parameters passed to function lcase(value)");
    }
    let string = params[0].to_string().to_lowercase();

    Ok(ValueType::String(string))
}

pub fn ucase<'a>(params: Vec<ValueType<'a>>, _: &mut Vm<'a>) -> Result<ValueType<'a>, &'a str> {
    if params.len() == 0 {
        return Err("Incorrect number of parameters passed to function ucase(value)");
    }
    let string = params[0].to_string().to_uppercase();

    Ok(ValueType::String(string))
}

pub fn instr<'a>(params: Vec<ValueType<'a>>, _: &mut Vm<'a>) -> Result<ValueType<'a>, &'a str> {
    let mut params_iter = params.iter();
    let str1 = params_iter.next();
    let str2 = params_iter.next();
    let start = params_iter.next();
    let compare = params_iter.next();

    if let None = str1 {
        return Err("Incorrect number of parameters passed to function instr(string1, string2, [start], [compare])");
    };

    if let None = str2 {
        return Err("Incorrect number of parameters passed to function instr(string1, string2, [start], [compare])");
    };

    let mut str1 = str1.unwrap().to_string();
    let mut str2 = str2.unwrap().to_string();

    if let Some(val) = compare {
        if let ValueType::Number(num) = val {
            if *num == 1.0 {
                // case insensitive compare
                str1 = str1.to_lowercase();
                str2 = str2.to_lowercase();
            }
        }
    }
    let start_index = if let Some(val) = start {
        let mut result: usize = 0;
        if let ValueType::Number(num) = val {
            if *num > 0.0 {
                result = *num as usize;
            }
        }
        result
    } else {
        0 as usize
    };

    if start_index >= str1.len() {
        return Ok(ValueType::Number(0.0));
    }

    let index = if start_index == 0 {
        str1.find(&str2)
    } else {
        let str2 = &str2[start_index..];
        str1.find(str2)
    };

    if let Some(index) = index {
        Ok(ValueType::Number((index - start_index + 1) as f64))
    } else {
        Ok(ValueType::Number(0.0))
    }
}

pub fn split<'a>(params: Vec<ValueType<'a>>, _: &mut Vm<'a>) -> Result<ValueType<'a>, &'a str> {
    let mut params_iter = params.iter();
    let string = params_iter.next();
    let delimiter = params_iter.next();
    let remove_empty = params_iter.next();
    //let compare = params_iter.next();

    if let None = string {
        return Err("Incorrect number of parameters passed to function split(string, delimiter, [remove_empty])");
    };

    if let None = delimiter {
        return Err("Incorrect number of parameters passed to function split(string, delimiter, [remove_empty])");
    };

    let remove_empty = if let Some(v) = remove_empty {
        if let ValueType::Boolean(v) = v {
            *v
        } else {
            return Err("Expect boolean for parameter 'remove_empty' of function split(string, delimiter, [remove_empty])");
        }
    } else {
        false
    };

    let string = string.unwrap().to_string();
    let delimiter = delimiter.unwrap().to_string();

    let parts = string.split(&delimiter);
    let result: Vec<ValueType> = if remove_empty {
        parts
            .filter(|x| !x.is_empty())
            .map(|x| ValueType::String(x.to_string()))
            .collect()
    } else {
        parts.map(|x| ValueType::String(x.to_string())).collect()
    };

    Ok(ValueType::Array(result))
}

#[cfg(test)]
mod tests {
    use super::round;

    #[test]
    fn test_round() {
        assert_eq!(round(123.45999, 2), 123.46);
        assert_eq!(round(123.454, 2), 123.45);
        assert_eq!(round(123.455, 2), 123.46);
    }
}
