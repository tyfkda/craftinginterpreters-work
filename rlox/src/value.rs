#![allow(non_snake_case)]

pub type Value = f64;

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
