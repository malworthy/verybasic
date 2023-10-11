use rand::seq::SliceRandom;
use rand::thread_rng;
use std::cmp::Ordering;

use super::{ValueType, Vm};

fn string_compare<'a>(op: &str, a: &str, b: &str) -> bool {
    match op {
        ">" => a > b,
        ">=" => a >= b,
        "<" => a < b,
        "<=" => a <= b,
        "=" | "==" => a == b,
        "<>" | "!=" => a != b,
        _ => false,
    }
}

fn filter_comparison(op: &str, a: &ValueType, b: &ValueType) -> bool {
    match a {
        ValueType::Number(a) => {
            if let ValueType::Number(ref b) = b {
                match op {
                    ">" => a > b,
                    ">=" => a >= b,
                    "<" => a < b,
                    "<=" => a <= b,
                    "=" | "==" => a == b,
                    "<>" | "!=" => a != b,
                    _ => false,
                }
            } else {
                false
            }
        }
        ValueType::Str(a) => match b {
            ValueType::Str(b) => string_compare(&op, a, b), // ValueType::Boolean(a == b),
            ValueType::String(b) => string_compare(&op, a, b.as_str()), //ValueType::Boolean(a == b),
            _ => false,
        },
        ValueType::String(a) => match b {
            ValueType::Str(b) => string_compare(&op, a.as_str(), b), //ValueType::Boolean(a == b),
            ValueType::String(b) => string_compare(&op, a.as_str(), b.as_str()), //ValueType::Boolean(a == b),
            _ => false,
        },
        ValueType::Boolean(a) => match b {
            ValueType::Boolean(b) => a == b,
            _ => false,
        },
        _ => false,
    }
}

fn filter_multi_comp(params: &Vec<ValueType>, x: &ValueType) -> bool {
    let mut iter = params.iter();
    let mut result = false;
    loop {
        if let Some(operator) = iter.next() {
            let operator = operator.to_string();

            if let Some(value) = iter.next() {
                result = result || filter_comparison(&operator, x, &value);
            } else {
                break;
            }
        } else {
            break;
        }
    }
    result
}

fn compare(a: &ValueType, b: &ValueType) -> std::cmp::Ordering {
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

pub fn array<'a>(params: Vec<ValueType<'a>>, _: &mut Vm<'a>) -> Result<ValueType<'a>, &'a str> {
    let mut array: Vec<ValueType> = Vec::new();
    for value in params {
        array.push(value)
    }
    Ok(ValueType::Array(array))
}

pub fn dim<'a>(params: Vec<ValueType<'a>>, _: &mut Vm<'a>) -> Result<ValueType<'a>, &'a str> {
    let size = if let ValueType::Number(num) = params[0] {
        if num < 1.0 {
            return Err("Parameter 1 of dim(size, [value]) must be a number 1 or greater");
        }
        (num + 1.0) as usize
    } else {
        return Err("Parameter 1 of dim(size, [value]) must be a number");
    };
    let value = if params.len() >= 2 {
        &params[1]
    } else {
        &ValueType::Number(0.0)
    };
    let mut array: Vec<ValueType> = Vec::with_capacity(size);
    for _ in 1..size {
        array.push(value.to_owned())
    }
    Ok(ValueType::Array(array))
}

pub fn filter<'a>(
    array: &mut ValueType<'a>,
    params: Vec<ValueType<'a>>,
) -> Result<ValueType<'a>, &'a str> {
    if params.len() < 2 {
        return Err("Incorrect number of parameters passed to filter()");
    }
    //let mut iter = params.iter();
    //let operator = iter.next().unwrap().to_string();
    //let value = iter.next().unwrap();
    let filtered = if let ValueType::Array(vec) = array {
        vec.into_iter().filter(|x| filter_multi_comp(&params, x))
    } else {
        return Err("Incorrect parameters passed to filter()");
    };
    let mut result: Vec<ValueType> = Vec::new();

    for item in filtered {
        result.push(item.clone());
    }

    Ok(ValueType::Array(result))
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

pub fn shuffle<'a>(params: Vec<ValueType<'a>>, _: &mut Vm<'a>) -> Result<ValueType<'a>, &'a str> {
    if params.len() < 1 {
        return Err("Incorrect parameters passed to shuffle(array)");
    }
    let array = &params[0];
    if let ValueType::Array(vec) = array {
        let mut result = vec.clone();
        result.shuffle(&mut thread_rng());
        return Ok(ValueType::Array(result));
    }
    Err("Incorrect parameters passed to shuffle(array)")
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

pub fn max<'a>(params: Vec<ValueType<'a>>, _: &mut Vm<'a>) -> Result<ValueType<'a>, &'a str> {
    if let Some(param) = params.first() {
        if let ValueType::Array(vec) = param {
            let result = vec.iter().max_by(|a, b| compare(a, b));
            match result {
                Some(result) => Ok(result.to_owned()),
                None => Ok(ValueType::Number(0.0)),
            }
        } else {
            Err("max(array) only works on arrays")
        }
    } else {
        Err("Incorrect number parameters passed to max(array)")
    }
}

pub fn find<'a>(params: Vec<ValueType<'a>>, _: &mut Vm<'a>) -> Result<ValueType<'a>, &'a str> {
    if let Some(param) = params.first() {
        if let ValueType::Array(vec) = param {
            let result = vec.iter().position(|a| {
                if let Ordering::Equal = compare(a, &params[1]) {
                    true
                } else {
                    false
                }
            });
            match result {
                Some(result) => Ok(ValueType::Number(result as f64)),
                None => Ok(ValueType::Number(-1.0)),
            }
        } else {
            Err("max(array) only works on arrays")
        }
    } else {
        Err("Incorrect number parameters passed to max(array)")
    }
}
