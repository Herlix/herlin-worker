use serde::{Deserialize, Deserializer, Serialize, Serializer};
use url::Url;
use wasm_bindgen::__rt::std::collections::HashMap;

#[derive(Deserialize, Serialize, Debug, PartialEq)]
/// Request struct
pub struct Request {
    /// Body of the request
    pub body: String,
    /// Method used, GET, PUT, POST, DELETE etc
    pub method: String,
    /// Headers of the request
    pub headers: HashMap<String, String>,
    #[serde(deserialize_with = "url_deserialize")]
    #[serde(serialize_with = "url_serialize")]
    /// Request destination URL
    pub url: Url,
}

impl Request {
    /// Insert headers into the request
    ///
    /// Overwrites old value of given key if it already exists
    pub fn add_header(&mut self, key: &str, value: &str) {
        if self.headers.contains_key(key) {
            self.headers.remove(key);
        }
        self.headers.insert(key.to_string(), value.to_string());
    }
}

#[derive(Serialize, Debug)]
/// Response struct
pub struct Response {
    /// HttpStatusCode
    pub status: u16,
    /// Headers of the response
    pub headers: HashMap<String, String>,
    /// Body of the response
    pub body: String,
}

impl Response {
    /// Insert headers into the request
    ///
    /// Overwrites old value of given key if it already exists
    pub fn add_header(&mut self, key: &str, value: &str) {
        self.headers.insert(key.to_string(), value.to_string());
    }
}

fn url_deserialize<'de, D>(d: D) -> Result<Url, D::Error>
where
    D: Deserializer<'de>,
{
    Deserialize::deserialize(d).map(|i: Option<String>| {
        let url = if let Some(v) = i { v } else { "".to_string() };
        Url::parse(url.as_str()).unwrap()
    })
}

fn url_serialize<S>(v: &Url, s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    s.serialize_str(v.as_str())
}

#[cfg(test)]
mod tests {
    use serde_json::from_str;
    use url::Url;

    use crate::models::{Request, Response};

    #[test]
    fn request_serialize_ok() {
        let mut req = Request {
            body: "".to_string(),
            method: "GET".to_string(),
            headers: Default::default(),
            url: Url::parse("https://test.se/").unwrap(),
        };
        req.add_header("content-type", "application/json");

        let res = serde_json::to_string(&req).unwrap();
        assert_eq!(res, "{\"body\":\"\",\"method\":\"GET\",\"headers\":{\"content-type\":\"application/json\"},\"url\":\"https://test.se/\"}".to_string());
    }

    #[test]
    fn request_deserialize_ok() {
        let input = "{\"body\":\"\",\"method\":\"GET\",\"headers\":{\"content-type\":\"application/json\"},\"url\":\"https://test.se/\"}";
        let mut expected = Request {
            body: "".to_string(),
            method: "GET".to_string(),
            headers: Default::default(),
            url: Url::parse("https://test.se/").unwrap(),
        };
        expected.add_header("content-type", "application/json");

        let result: Request = from_str(input).unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn response_serialize_ok() {
        let mut req = Response {
            body: "".to_string(),
            headers: Default::default(),
            status: 200,
        };
        req.add_header("content-type", "application/json");

        let res = serde_json::to_string(&req).unwrap();
        assert_eq!(
            res,
            "{\"status\":200,\"headers\":{\"content-type\":\"application/json\"},\"body\":\"\"}"
                .to_string()
        );
    }
}
