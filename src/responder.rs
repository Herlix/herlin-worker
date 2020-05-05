use crate::documents::UserScore;
use crate::kv::CloudFlareKV;
use crate::request::{Request, Response};
use wasm_bindgen::__rt::std::collections::HashMap;
use wasm_bindgen::__rt::std::io;

pub async fn respond(req: Request) -> Result<Response, io::Error> {
    // Middleware
    // Url parse
    // Return result

    let path = req.url.path_segments().map(|x| x.collect::<Vec<&str>>());
    let params: HashMap<String, String> = req
        .url
        .query_pairs()
        .map(|x| (x.0.to_string(), x.1.to_string()))
        .collect();

    let get = UserScore::get("2").await?;

    Ok(Response {
        status: 200,
        body: format!(
            "fetched nr 2{:?} ->\nfrom path {:?} -> with query params {:?}",
            serde_json::to_string(&get),
            path,
            params
        ),
        headers: req.headers,
    })
}
