use serde::{Deserialize, Deserializer, Serialize, Serializer};
use url::Url;
use wasm_bindgen::__rt::std::collections::HashMap;

#[derive(Deserialize, Serialize, Debug, PartialEq)]
pub struct Request {
    pub body: String,
    pub method: String,
    pub headers: HashMap<String, String>,

    #[serde(deserialize_with = "url_deserialize")]
    #[serde(serialize_with = "url_serialize")]
    pub url: Url,
}

impl Request {
    pub fn add_header(&mut self, key: &str, value: &str) {
        self.headers.insert(key.to_string(), value.to_string());
    }
}

#[derive(Serialize, Debug)]
pub struct Response {
    pub status: u16,
    pub headers: HashMap<String, String>,
    pub body: String,
}

impl Response {
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
    use crate::models::{Request, Response};
    use serde_json::from_str;
    use url::Url;

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
