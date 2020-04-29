use js_sys::Promise;
use serde::{Deserialize, Serialize};
use serde_json::from_str;
use wasm_bindgen::__rt::std::io;
use wasm_bindgen::__rt::std::io::ErrorKind;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;

type Result<T> = wasm_bindgen::__rt::std::result::Result<T, io::Error>;

#[derive(Serialize, Deserialize)]
pub struct UserScore {
    pub email: String,
    pub score: u8,
}

impl UserScore {
    pub async fn get(key: &str) -> Result<Self> {
        let result = call_js(js_get_userScore(key)).await?.as_string().unwrap();
        let result = from_str(result.as_str())?;
        Ok(result)
    }

    pub async fn put(key: &str, value: Self) -> Result<Self> {
        let value = serde_json::to_string(&value)?;
        if let Ok(_) = call_js(js_put_userScore(key, value.as_str())).await {
            Self::get(key).await
        } else {
            Err(std::io::Error::new(
                ErrorKind::InvalidInput,
                format!("Could not find item with id {}", key),
            ))
        }
    }

    pub async fn delete(key: &str) -> Result<bool> {
        match call_js(js_delete_userScore(key)).await {
            Ok(_) => Ok(true),
            Err(e) => Err(e),
        }
    }
}

async fn call_js(promise: Promise) -> Result<JsValue> {
    let result = JsFuture::from(promise).await.map_err(|e| {
        io::Error::new(
            ErrorKind::InvalidInput,
            e.as_string()
                .unwrap_or("Fatal error while accessing JS".to_string()),
        )
    })?;
    Ok(result)
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = UserScore, js_name = get)]
    fn js_get_userScore(key: &str) -> Promise;

    #[wasm_bindgen(js_namespace = UserScore, js_name = put)]
    fn js_put_userScore(key: &str, value: &str) -> Promise;

    #[wasm_bindgen(js_namespace = UserScore, js_name = delete)]
    fn js_delete_userScore(key: &str) -> Promise;
}
