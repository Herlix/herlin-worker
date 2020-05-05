use async_trait::async_trait;
use serde::{Deserialize, Serialize};

pub trait JsKvPromise {
    fn js_get(key: &str) -> js_sys::Promise;
    fn js_delete(key: &str) -> js_sys::Promise;
    fn js_put(key: &str, value: &str) -> js_sys::Promise;
}

#[async_trait(?Send)]
pub trait CloudFlareKV<'de, T>
where
    T: JsKvPromise + Serialize + Deserialize<'de>,
{
    async fn get(
        key: &str,
    ) -> wasm_bindgen::__rt::std::result::Result<T, wasm_bindgen::__rt::std::io::Error>;

    async fn put(
        key: &str,
        value: T,
    ) -> wasm_bindgen::__rt::std::result::Result<T, wasm_bindgen::__rt::std::io::Error>;

    async fn delete(
        key: &str,
    ) -> wasm_bindgen::__rt::std::result::Result<(), wasm_bindgen::__rt::std::io::Error>;
}

pub async fn call_js(
    promise: js_sys::Promise,
) -> wasm_bindgen::__rt::std::result::Result<
    wasm_bindgen::JsValue,
    wasm_bindgen::__rt::std::io::Error,
> {
    let result = wasm_bindgen_futures::JsFuture::from(promise)
        .await
        .map_err(|e| {
            wasm_bindgen::__rt::std::io::Error::new(
                wasm_bindgen::__rt::std::io::ErrorKind::Other,
                e.as_string().unwrap(),
            )
        })?;
    Ok(result)
}
