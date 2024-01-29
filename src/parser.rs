use std::collections::HashMap;

use crate::{
    tokenizer::{self, Tokenizer},
    value::Value,
};

pub fn parse<'a>(s: &'a str) -> Value {
    if s.is_empty() {
        return Value::default();
    }
    let mut toker = tokenizer::Tokenizer::new(s);
    process(&mut toker);

    Value::default()
}

pub fn process<'a>(toker: &'a mut Tokenizer) -> Value {
    toker.get_char();
    if toker.ch.as_ref().unwrap().ch == '{' {
        toker.skip_bc();
        if toker.ch.as_ref().unwrap().ch == '}' {
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
                toker.skip_bc();
                let value: Value = process(toker);
                map.insert(key, value.clone());
            } else {
                panic!("grammar error: no : comma")
            };

            toker.skip_bc();
            if toker.ch.as_ref().unwrap().ch == ',' {
                toker.get_char();
            }
        }
    }

    Value::default()
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
