use serde::{Deserialize, Serialize};
use std::fmt::Display;

pub use wasm_bindgen::JsValue;

#[derive(Deserialize, Serialize, Debug)]
pub struct JSError {
    pub msg: String,
}

impl Display for JSError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Inner: {}", self.msg)
    }
}

impl From<serde_json::Error> for JSError {
    fn from(e: serde_json::Error) -> Self {
        JSError { msg: e.to_string() }
    }
}

impl From<JsValue> for JSError {
    fn from(e: JsValue) -> Self {
        JSError {
            msg: e.as_string().unwrap(),
        }
    }
}

impl From<JSError> for JsValue {
    fn from(e: JSError) -> Self {
        JsValue::from_str(e.msg.as_str())
    }
}
