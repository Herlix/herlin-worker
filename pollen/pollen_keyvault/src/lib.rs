use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::{future::Future, pin::Pin};

pub use wasm_bindgen::JsValue;

#[derive(Deserialize, Serialize)]
pub struct JSError {
    pub msg: String,
}

impl From<serde_json::Error> for JSError {
    fn from(e: serde_json::Error) -> Self {
        JSError { msg: e.to_string() }
    }
}

pub trait CloudFlareKV {
    fn get<'a>(key: &'a str) -> Pin<Box<dyn Future<Output = Result<Self, JSError>> + 'a>>
    where
        Self: DeserializeOwned + 'a;

    fn put<'a>(
        key: &'a str,
        value: Self,
    ) -> Pin<Box<dyn Future<Output = Result<Self, JSError>> + 'a>>
    where
        Self: Serialize + DeserializeOwned + 'a;

    fn delete<'a>(key: &'a str) -> Pin<Box<dyn Future<Output = Result<(), JSError>> + 'a>>;
}
