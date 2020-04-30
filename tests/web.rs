#![cfg(target_arch = "wasm32")]

use herlin_worker::get_response;
use herlin_worker::request::{Method, Request, Response};
use url::Url;
use wasm_bindgen::JsValue;
use wasm_bindgen::__rt::std::collections::HashMap;
use wasm_bindgen_futures::{future_to_promise, JsFuture};
use wasm_bindgen_test::*;

#[wasm_bindgen_test]
async fn get_response_no_failure() {
    let input = input_request();
    let expected = expected_response();

    let promise = future_to_promise(get_response(input.clone()));
    let result = JsFuture::from(promise).await.unwrap();
    assert_eq!(result.as_string(), expected.as_string());
}
