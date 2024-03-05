use regex::Regex;

use crate::{
    tokenizer::{self, PosRange, Tokenizer},
    value::Value,
};
use std::collections::HashMap;


/// 解析函数,用于解析json
pub fn parse<'a>(s: &'a str) -> Value {
    //continuous empty string
    if s.is_empty() {
        return Value::default();
    }
    let mut toker = tokenizer::Tokenizer::new(s);
    toker.get_char();
    process(&mut toker)
}

pub fn process<'a>(toker: &'a mut Tokenizer) -> Value {
    if toker.ch.is_none() {
        return Value::default();
    }
    toker.skip_bc();
    // println!("xxx: {:?}", toker.ch);
    if toker.ch.as_ref().unwrap().ch == '{' {
        return proc_object(toker);
    }
    if toker.ch.as_ref().unwrap().ch == '"' {
        return proc_string(toker);
    }
    if toker.ch.as_ref().unwrap().ch == '[' {
        return proc_array(toker);
    }
    if toker.is_letter() {
        return proc_bool_null(toker);
    }
    if toker.is_digit() || toker.ch.as_ref().unwrap().ch == '-' {
        return proc_number(toker);
    }
    panic!(
        "grammar error: should be Object/Array/String/bool/null/Number, current ch is:{:?}",
        toker.ch
    )
}

fn proc_bool_null<'a>(toker: &'a mut Tokenizer) -> Value {
    let step_n;
    if toker.ch.as_ref().unwrap().ch == 't' || toker.ch.as_ref().unwrap().ch == 'n' {
        step_n = 4;
    } else if toker.ch.as_ref().unwrap().ch == 'f' {
        step_n = 5;
    } else {
        panic!("grammar error: should be true/false or null")
    }
    for _ in 0..step_n {
        toker.concat();
        toker.get_char();
    }
    let pos_range = toker.get_str_token();
    return Value::string(pos_range);
}

fn proc_number<'a>(toker: &'a mut Tokenizer) -> Value {
    toker.skip_bc();
    if toker.ch.as_ref().unwrap().ch == '-' {
        toker.concat();
        toker.get_char();
        if !toker.is_digit() {
            panic!("grammar error: -x x should be digit")
        }
    }
    if toker.ch.as_ref().unwrap().ch == '0' {
        toker.concat();
        toker.get_char();
    } else {
        while toker.is_digit() {
            toker.concat();
            toker.get_char();
        }
    }
    if toker.ch.is_some() && toker.ch.as_ref().unwrap().ch == '.' {
        toker.concat();
        toker.get_char();
        while toker.is_digit() {
            toker.concat();
            toker.get_char();
        }
    }
    if toker.ch.is_some()
        && (toker.ch.as_ref().unwrap().ch == 'E' || toker.ch.as_ref().unwrap().ch == 'e')
    {
        toker.concat();
        toker.get_char();
        if toker.ch.as_ref().unwrap().ch == '-' || toker.ch.as_ref().unwrap().ch == '+' {
            toker.concat();
            toker.get_char();
        }
        while toker.is_digit() {
            toker.concat();
            toker.get_char();
        }
    }

    let pos_range = toker.get_str_token();
    Value::string(pos_range)
}

fn proc_array<'a>(toker: &'a mut Tokenizer) -> Value {
    toker.get_char();
    toker.skip_bc();
    if toker.ch.as_ref().unwrap().ch == ']' {
        toker.get_char();
        return Value::Array(vec![]);
    }
    let mut arr = vec![];
    while toker.ch.as_ref().unwrap().ch != ']' {
        let v = process(toker);
        arr.push(v);
        toker.skip_bc();
        if toker.ch.as_ref().unwrap().ch == ',' {
            toker.get_char();
        }
    }
    toker.get_char();

    Value::Array(arr)
}

fn proc_string<'a>(toker: &'a mut Tokenizer) -> Value {
    toker.get_char();
    if toker.ch.as_ref().unwrap().ch == '"' {
        let offset = toker.ch.as_ref().unwrap().offset;
        let pos_range = PosRange::new(offset, offset);
        toker.get_char();
        return Value::String(pos_range);
    }
    while toker.ch.as_ref().unwrap().ch != '"' {
        if toker.ch.as_ref().unwrap().ch == '\\' {
            toker.concat();
            toker.get_char();
        }
        toker.concat();
        toker.get_char();
    }
    toker.get_char();
    Value::string(toker.get_str_token())
}

fn proc_object<'a>(toker: &'a mut Tokenizer) -> Value {
    toker.get_char();
    toker.skip_bc();
    if toker.ch.as_ref().unwrap().ch == '}' {
        toker.get_char();
        let map = HashMap::new();
        return Value::object(map);
    }
    let mut map = HashMap::with_capacity(32);
    while toker.ch.as_ref().unwrap().ch != '}' {
        toker.skip_bc();
        let key = match process(toker) {
            Value::String(key) => key,
            _ => panic!("grammar error: key type error!"),
        };
        toker.skip_bc();
        if toker.ch.as_ref().unwrap().ch == ':' {
            toker.get_char();
            let value: Value = process(toker);
            map.insert(key, value);
        } else {
            panic!("grammar error: need colon symbol : , not: {:?}", toker.ch)
        }

        toker.skip_bc();
        if toker.ch.as_ref().unwrap().ch == ',' {
            toker.get_char();
        }
    }
    toker.get_char();
    Value::object(map)
}

// pub fn process<'a>(toker: &'a mut Tokenizer) -> Value<'a> {
//     toker.get_char();
//     if toker.ch.as_ref().unwrap().ch == '{' {
//         toker.skip_bc();
//         if toker.ch.as_ref().unwrap().ch == '}' {
//             let map = HashMap::new();
//             return Value::object(map);
//         }
//         let mut map = HashMap::with_capacity(32);
//         while toker.ch.as_ref().unwrap().ch != '}' {
//             toker.skip_bc();
//             let key = match process(toker) {
//                 Value::String(key) => key,
//                 _ => panic!("grammar error: key type error!"),
//             };
//             toker.skip_bc();
//             if toker.ch.as_ref().unwrap().ch == ':' {
//                 toker.skip_bc();
//                 let value: Value<'_> = process(toker);
//使用保存&str的方式，在while循环里解析保存返回数据时候，会遇到生命周期的问题！
//这里解决，要么使用包装的方式，包装一下，要么只能怪
//                 map.insert(key, value.clone());
//             } else {
//                 panic!("grammar error: no : comma")
//             };

//             toker.skip_bc();
//             if toker.ch.as_ref().unwrap().ch == ',' {
//                 toker.get_char();
//             }
//         }
//     }

//     Value::default()
// }
