use std::collections::HashMap;


pub enum Value<'a> {
    Null(&'a str),
    Bool(&'a str),
    Number(&'a str),
    String(&'a str),
    Array(Vec<Value<'a>>),
    Object(HashMap<&'a str, Value<'a>>),
}