pub mod models;
mod utils;

use crate::models::{Request, Response};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;

#[wasm_bindgen]
pub async fn get_response(request: JsValue) -> Result<JsValue, JsValue> {
    let req = request.into_serde().map_err(|e| e.to_string())?;
    let res = respond(req).await.map_err(|e| e.to_string())?;
    let res = JsValue::from_serde(&res).map_err(|e| e.to_string())?;
    Ok(res)
}

async fn respond(req: Request) -> Result<Response, Box<dyn std::error::Error>> {
    Ok(Response {
        status: 200,
        body: req.body,
        headers: req.headers,
    })
}
