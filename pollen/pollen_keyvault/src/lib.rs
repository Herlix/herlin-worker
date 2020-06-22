#[warn(missing_debug_implementations, rust_2018_idioms, missing_docs)]
use serde::{de::DeserializeOwned, Serialize};
use std::{future::Future, pin::Pin};

pub use js_error::JSError;
pub use wasm_bindgen::JsValue;
mod js_error;

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
