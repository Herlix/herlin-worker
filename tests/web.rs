//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]
// wasm_bindgen_test_configure!(run_in_browser);

use herlin_worker::get_response;
use herlin_worker::request::{Request, Response};
use url::Url;
use wasm_bindgen::JsValue;
use wasm_bindgen::__rt::std::collections::HashMap;
use wasm_bindgen_futures::{future_to_promise, JsFuture};
use wasm_bindgen_test::*;

#[cfg(test)]
pub fn input_request() -> JsValue {
    let mut req = Request {
        body: "".to_string(),
        method: "GET".to_string(),
        headers: HashMap::default(),
        url: Url::parse("https://test.se/").unwrap(),
    };
    req.add_header("content-type", "application/json");
    JsValue::from_serde(&req).unwrap()
}

#[cfg(test)]
pub fn expected_response() -> JsValue {
    let mut req = Response {
        status: 200,
        body: "".to_string(),
        headers: HashMap::default(),
    };
    req.add_header("content-type", "application/json");
    JsValue::from_serde(&req).unwrap()
}

#[wasm_bindgen_test]
async fn get_response_no_failure() {
    let input = input_request();
    let expected = expected_response();

    let promise = future_to_promise(get_response(input.clone()));
    let result = JsFuture::from(promise).await.unwrap();
    assert_eq!(result.as_string(), expected.as_string());
}
