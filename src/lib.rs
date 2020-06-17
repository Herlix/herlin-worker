#[warn(missing_debug_implementations, rust_2018_idioms, missing_docs)]
mod models;
mod utils;

use crate::models::UserScore;
use cf_kv::CloudFlareKV;
use herlin_web::{
    body::Body, deserialize_request, request::HttpRequest, response::HttpResponse, App,
};
use log::info;
use log::Level;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;

/// Get response from WASM
/// Takes the request and produces a response
#[wasm_bindgen]
pub async fn get_response(request: JsValue) -> Result<JsValue, JsValue> {
    console_log::init_with_level(Level::Debug).expect("Could not init logger");
    let req = deserialize_request!(&request)?;

    let mut app = App::new(req);
    app.reg("userscore", get_user_score);

    let response = app.response().await;
    Ok(JsValue::from_serde(&response).map_err(|e| JsValue::from(e.to_string()))?)
}

async fn get_user_score(req: HttpRequest) -> HttpResponse {
    info!("id: {:?}", &req.path);

    let id = req.path.get("id").unwrap_or("2");
    info!("id: {}", &id);
    let user_score = UserScore::get(id).await.unwrap();
    let mut req = HttpResponse::default();
    req.set_body(Body::Message(serde_json::to_string(&user_score).unwrap()));
    req
}
