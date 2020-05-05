use crate::documents::{UserInfo, UserScore};
use crate::request::{Request, Response};
use wasm_bindgen::__rt::std::collections::HashMap;
use wasm_bindgen::__rt::std::error::Error;
use crate::kv::CloudFlareKV;

pub async fn respond(req: Request) -> Result<Response, Box<dyn Error>> {
    // Middleware
    // Url parse
    // Return result

    let path = req.url.path_segments().map(|x| x.collect::<Vec<&str>>());
    let params: HashMap<String, String> = req
        .url
        .query_pairs()
        .map(|x| (x.0.to_string(), x.1.to_string()))
        .collect();
    
    UserInfo::put(
        "1",
        UserInfo {
            email: "alexander.herlin@outlook.com".to_string(),
            name: "Alexander".to_string(),
        },
    )
    .await?;

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn stub() {
        assert_eq!(1, 1);
    }
}
