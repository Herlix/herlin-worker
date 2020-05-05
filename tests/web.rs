#![cfg(target_arch = "wasm32")]

#[cfg(test)]
mod integration_tests {
    use herlin_worker::get_response;
    use herlin_worker::request::{Method, Request, Response};
    use url::Url;
    use wasm_bindgen::JsValue;
    use wasm_bindgen::__rt::std::collections::HashMap;
    use wasm_bindgen_futures::{future_to_promise, JsFuture};
    use wasm_bindgen_test::*;

    pub fn input_request() -> JsValue {
        let mut req = Request {
            body: "".to_string(),
            method: Method::GET,
            headers: HashMap::default(),
            url: Url::parse("https://test.se/").unwrap(),
        };
        req.headers
            .insert("content-type".to_string(), "application/json".to_string());
        JsValue::from_serde(&req).unwrap()
    }

    pub fn expected_response() -> JsValue {
        let mut req = Response {
            status: 200,
            body: "".to_string(),
            headers: HashMap::default(),
        };
        req.headers
            .insert("content-type".to_string(), "application/json".to_string());
        JsValue::from_serde(&req).unwrap()
    }

    #[wasm_bindgen_test]
    async fn get_response_no_failure() {
        // todo: Mock the DB connection

        let input = input_request();
        let expected = expected_response();

        let promise = future_to_promise(get_response(input.clone()));
        let result = JsFuture::from(promise).await.unwrap();
        assert_eq!(result.as_string(), expected.as_string());
    }
}
