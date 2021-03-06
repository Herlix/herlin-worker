#![cfg(target_arch = "wasm32")]

#[cfg(test)]
mod integration_tests {
    use actix_router::{Path, Url};
    use http::{HeaderMap, HeaderValue, Method, Uri};
    use pollen::{body::Body, request::HttpRequest};
    use serde::Serialize;
    use std::convert::TryFrom;
    use wasm_bindgen::JsValue;
    use wasm_bindgen::__rt::std::collections::HashMap;
    use wasm_bindgen_test::*;

    #[wasm_bindgen_test]
    async fn deserialize_request_empty() {
        let j = JsValue::from_str("");
        let m = HttpRequest::from_js_value(j);
        assert!(m.is_err());
    }

    #[wasm_bindgen_test]
    async fn deserialize_request_valid() {
        let mut headers = HeaderMap::with_capacity(1);
        headers.insert(
            http::header::ACCEPT,
            HeaderValue::try_from(
                "text/html,application/xhtml+xml,application/xml;q=0.9,image/webp,*/*;q=0.8",
            )
            .unwrap(),
        );

        let expected = HttpRequest {
            method: Method::GET,
            headers,
            path: Path::new(Url::new("https://example.com/".parse::<Uri>().unwrap())),
            body: Body::Empty,
        };

        let mut h = HashMap::with_capacity(1);
        h.insert(
            "accept",
            "text/html,application/xhtml+xml,application/xml;q=0.9,image/webp,*/*;q=0.8",
        );

        let js_value = JsValue::from_serde(&TestHttpRequest {
            method: "GET",
            headers: h,
            path: "https://example.com/",
            body: "",
        })
        .unwrap();

        let res = HttpRequest::from_js_value(js_value);
        assert_eq!(format!("{:?}", res.unwrap()), format!("{:?}", expected));
    }

    #[derive(Serialize)]
    struct TestHttpRequest<'a> {
        method: &'a str,
        headers: HashMap<&'a str, &'a str>,
        path: &'a str,
        body: &'a str,
    }
}
