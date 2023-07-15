use crate::vm::ValueType;

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

pub fn right(params: Vec<ValueType>) -> Result<ValueType, &str> {
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

pub fn str(params: Vec<ValueType>) -> Result<ValueType, &str> {
    if params.len() == 0 {
        return Err("Incorrect number of parameters passed to function str(value)");
    }
    let string = params[0].to_string();

    Ok(ValueType::String(string))
}

pub fn lcase(params: Vec<ValueType>) -> Result<ValueType, &str> {
    if params.len() == 0 {
        return Err("Incorrect number of parameters passed to function lcase(value)");
    }
    let string = params[0].to_string().to_lowercase();

    Ok(ValueType::String(string))
}

pub fn ucase(params: Vec<ValueType>) -> Result<ValueType, &str> {
    if params.len() == 0 {
        return Err("Incorrect number of parameters passed to function ucase(value)");
    }
    let string = params[0].to_string().to_uppercase();

    Ok(ValueType::String(string))
}

pub fn instr(params: Vec<ValueType>) -> Result<ValueType, &str> {
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
