use super::value::{AS_OBJ, Value};

#[derive(Clone, Debug, PartialEq)]
pub enum Obj {
    STRING(ObjString),
}

#[derive(Clone, Debug, PartialEq)]
pub struct ObjString {
    pub s: String,
}

impl ObjString {
    pub fn str(&self) -> &str {
        &self.s
    }
}

pub trait ObjTrait<T> {
    fn VAL(object: T) -> Value;
}

pub fn AS_STRING(value: &Value) -> Option<&ObjString> {
    if let Some(obj) = AS_OBJ(value) {
        if let Obj::STRING(s) = obj {
            Some(s)
        } else {
            None
        }
    } else {
        None
    }
}

pub fn AS_STRING_MUT(value: &mut Value) -> Option<&mut ObjString> {
    if let Value::OBJ(obj) = value {
        if let Obj::STRING(s) = obj {
            Some(s)
        } else {
            None
        }
    } else {
        None
    }
}

pub fn IS_STRING(value: &Value) -> bool {
    AS_STRING(value).is_some()
}

pub fn copyString<'a>(chars: &'a str, length: usize) -> ObjString {
    allocateString(String::from(&chars[0..length]))
}

pub fn takeString(string: String) -> ObjString {
    allocateString(string)
}

pub fn allocateString(string: String) -> ObjString {
    ObjString {s: string}
}

pub fn printObject(obj: &Obj) {
    match obj {
        Obj::STRING(s)   => { print!("{}", s.str()); }
    }
}
