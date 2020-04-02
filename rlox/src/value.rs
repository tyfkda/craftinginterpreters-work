#![allow(non_snake_case)]

use super::object::{printObject, Obj, ObjString, ObjTrait};

#[derive(Clone, Debug, PartialEq)]
pub enum Value {
    BOOL(bool),
    NIL,
    NUMBER(f64),
    OBJ(Obj)
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

pub fn IS_NUMBER(value: &Value) -> bool {
    AS_NUMBER(value).is_some()
}

pub fn AS_OBJ(value: &Value) -> Option<&Obj> {
    if let Value::OBJ(obj) = value {
        Some(obj)
    } else {
        None
    }
}

pub fn IS_OBJ(value: &Value) -> bool {
    AS_OBJ(value).is_some()
}

impl ObjTrait<ObjString> for Obj {
    fn VAL(object: ObjString) -> Value {
        Value::OBJ(Obj::STRING(object))
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

pub fn valuesEqual(a: &Value, b: &Value) -> bool {
    *a == *b
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
        Value::OBJ(obj)  => { printObject(obj); }
    }
}
