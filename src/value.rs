use std::{collections::HashMap, vec};

use crate::tokenizer::PosRange;

//使用&str，在解析的时候，while循环里会遇到生命周期的问题
// pub enum JValue<'a> {
//     Null(&'a str),
//     Bool(&'a str),
//     Number(&'a str),
//     String(&'a str),
//     Array(Vec<Value<'a>>),
//     Object(HashMap<&'a str, Value<'a>>),
// }

// impl<'a> JValue<'a> {
//     pub fn object(map: HashMap<&'a str, Value<'a>>) -> Value<'a> {
//         Self::Object(map)
//     }
//     pub fn array(arr: Vec<Value<'a>>) -> Value<'a> {
//         Self::Array(arr)
//     }
//     pub fn string(s: &'a str) -> Value<'a> {
//         Self::String(s)
//     }
// }

#[derive(Clone, Debug)]
pub enum Value {
    String(PosRange),
    Array(Vec<Value>),
    Object(HashMap<PosRange, Value>),
}

impl Value {
    pub fn object(map: HashMap<PosRange, Value>) -> Value {
        Self::Object(map)
    }
    pub fn array(arr: Vec<Value>) -> Value {
        Self::Array(arr)
    }
    pub fn string(s: PosRange) -> Value {
        Self::String(s)
    }

    pub fn _show(&self, data: &str) -> String {
        let mut ret = String::new();
        match self {
            Value::String(range) => {
                ret.push('"');
                ret.push_str(&data[range.start..range.end + 1]);
                ret.push('"');
            }
            Value::Object(map) => {
                ret.push('{');
                for (k, v) in map.iter() {
                    let key = data[k.start..k.end + 1].to_string();
                    let value = v._show(data);
                    ret.push_str(&format!("\"{}\":{},", key, value));
                }
                ret.pop();
                ret.push('}');
            }
            Value::Array(arr) => {
                ret.push('[');
                for v in arr.iter() {
                    let v = v._show(data);
                    ret.push_str(&v);
                    ret.push(',');
                }
                ret.pop();
                ret.push(']');
            }
        }
        ret
    }

    pub fn show(&self, data: &str) {
        let ret = self._show(data);
        println!("{:?}", ret);
    }

    pub fn to_string(&self, data: &str) -> String {
        self._show(data)
    }
}

impl Default for Value {
    fn default() -> Self {
        Self::String(PosRange::default())
    }
}
