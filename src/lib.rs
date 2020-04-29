mod kv;
#[warn(missing_debug_implementations, rust_2018_idioms, missing_docs)]
/// Request & Response models
pub mod request;
mod utils;

use crate::kv::user_score::UserScore;
use request::{Request, Response};
use wasm_bindgen::__rt::std::collections::HashMap;
use wasm_bindgen::__rt::std::error::Error;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;

#[wasm_bindgen]
pub async fn get_response(request: JsValue) -> Result<JsValue, JsValue> {
    let req = request
        .into_serde()
        .map_err(|e| JsValue::from_str(e.to_string().as_str()))?;
    let res = respond(req)
        .await
        .map_err(|e| JsValue::from_str(e.to_string().as_str()))?;
    let res = JsValue::from_serde(&res).map_err(|e| JsValue::from_str(e.to_string().as_str()))?;
    Ok(res)
}

async fn respond(req: Request) -> Result<Response, Box<dyn Error>> {
    // Middleware
    // Url parse
    // Return result

    let path = req.url.path_segments().map(|x| x.collect::<Vec<&str>>());
    let params: HashMap<String, String> = req
        .url
        .query_pairs()
        .map(|x| (x.0.to_lowercase(), x.1.to_lowercase()))
        .collect();

    let del = UserScore::delete("1").await?;
    let put = UserScore::put(
        "2",
        UserScore {
            email: "test@test1.se".to_string(),
            score: 0,
        },
    )
    .await?;

    let get = UserScore::get("2").await?;

    Ok(Response {
        status: 200,
        body: format!(
            "Deleted nr 1: {:?} -> Created number 2{:?} -> fetched nr 2{:?} ->\nfrom path {:?} -> with query params {:?}",
            del,
            serde_json::to_string(&put),
            serde_json::to_string(&get),
            path,
            params
        ),
        headers: req.headers,
    })
}
