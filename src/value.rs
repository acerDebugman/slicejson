use std::collections::HashMap;

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
}

impl Default for Value {
    fn default() -> Self {
        Self::String(PosRange::default())
    }
}
