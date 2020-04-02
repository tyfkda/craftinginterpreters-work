#![allow(non_snake_case)]

#[derive(Clone, Debug)]
pub enum Value {
    BOOL(bool),
    NIL,
    NUMBER(f64),
}

pub fn AS_BOOL(value: &Value) -> Option<bool> {
    if let Value::BOOL(b) = value {
        Some(*b)
    } else {
        None
    }
}

pub fn IS_NIL(value: &Value) -> bool {
    if let Value::NIL = value {
        true
    } else {
        false
    }
}

pub fn AS_NUMBER(value: &Value) -> Option<f64> {
    if let Value::NUMBER(n) = value {
        Some(*n)
    } else {
        None
    }
}

pub struct ValueArray {
    pub values: Vec<Value>,
}

impl ValueArray {
    pub fn count(&self) -> usize {
        self.values.len()
    }
}

pub fn initValueArray() -> ValueArray {
    ValueArray {
        values: vec![],
    }
}

pub fn writeValueArray(array: &mut ValueArray, value: Value) {
    array.values.push(value);
}

pub fn printValue(value: &Value) {
    match value {
        Value::BOOL(b)   => { print!("{}", b); }
        Value::NIL       => { print!("nil"); }
        Value::NUMBER(n) => { print!("{}", n); }
    }
}
