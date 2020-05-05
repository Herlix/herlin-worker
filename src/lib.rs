#[warn(missing_debug_implementations, rust_2018_idioms, missing_docs)]

/// Default Request & Response objects
pub mod request;

mod documents;
mod kv;
mod responder;
mod utils;

use request::Request;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;

/// Get response from WASM
/// Takes the request and produces a response
#[wasm_bindgen]
pub async fn get_response(request: JsValue) -> Result<JsValue, JsValue> {
    let req: Request = request
        .into_serde()
        .map_err(|e| JsValue::from_str(e.to_string().as_str()))?;

    let res = responder::respond(req)
        .await
        .map_err(|e| JsValue::from_str(e.to_string().as_str()))?;

    let res = JsValue::from_serde(&res).map_err(|e| JsValue::from_str(e.to_string().as_str()))?;
    Ok(res)
}
