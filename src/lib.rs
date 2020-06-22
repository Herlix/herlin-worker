#[warn(missing_debug_implementations, rust_2018_idioms, missing_docs)]
mod models;
mod utils;

use crate::models::UserScore;
use log::info;
use log::Level;
use pollen::{body::Body, request::HttpRequest, response::HttpResponse, App, CloudFlareKV};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;

/// Get response from WASM
/// Takes the request and produces a response
#[wasm_bindgen]
pub async fn get_response(request: JsValue) -> Result<JsValue, JsValue> {
    console_log::init_with_level(Level::Debug).expect("Could not init logger");
    let req = HttpRequest::from(request);

    let mut app = App::new(req);
    app.reg("userscore", get_user_score);

    let response = app.response().await;
    Ok(JsValue::from_serde(&response).map_err(|e| JsValue::from(e.to_string()))?)
}

async fn get_user_score(req: HttpRequest) -> HttpResponse {
    info!("id: {:?}", &req.path);

    let id = req.path.get("id").unwrap_or("2");
    info!("id: {}", &id);
    let us = match UserScore::get(id).await {
        Ok(u) => serde_json::to_string(&u).unwrap(),
        Err(e) => String::from(e.msg),
    };
    let mut req = HttpResponse::default();
    req.set_body(Body::Message(us));
    req
}
