#![cfg(target_arch = "wasm32")]

#[cfg(test)]
mod integration_tests {
    // use herlin_worker::get_response;
    // use herlin_worker::request::{Method, Request, Response};
    // use url::Url;
    // use wasm_bindgen::prelude::*;
    // use wasm_bindgen::JsValue;
    use wasm_bindgen_test::*;

    #[wasm_bindgen_test]
    async fn stub() {
        // TODO: Mock JS object

        // let req = Request {
        //     body: "Body".to_string(),
        //     method: Method::GET,
        //     headers: Default::default(),
        //     url: Url::parse("https://duckduckgo.com/?smooth=yes").unwrap(),
        // };
        // let res = get_response(JsValue::from_serde(&req).unwrap())
        //     .await
        //     .unwrap();
        //
        // assert_eq!(
        //     JsValue::from_serde(&Response {
        //         status: 200,
        //         headers: Default::default(),
        //         body: "Body".to_string()
        //     })
        //     .unwrap(),
        //     res
        // );
        assert_eq!(4, 4);
    }
}
