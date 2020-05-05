use serde::{Deserialize, Deserializer, Serialize, Serializer};
use url::Url;
use wasm_bindgen::__rt::core::result::Result;
use wasm_bindgen::__rt::std::collections::HashMap;

/// HTTP method
#[derive(Debug, PartialEq)]
pub enum Method {
    /// GET
    GET,
    /// PUT
    PUT,
    /// POST
    POST,
    /// DELETE
    DELETE,
    /// PATCH
    PATCH,
}

impl Serialize for Method {
    fn serialize<S>(&self, s: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
    where
        S: Serializer,
    {
        s.serialize_str(match &self {
            Method::DELETE => "DELETE",
            Method::GET => "GET",
            Method::PUT => "PUT",
            Method::POST => "POST",
            Method::PATCH => "PATCH",
        })
    }
}

impl<'de> Deserialize<'de> for Method {
    fn deserialize<D>(d: D) -> Result<Self, <D as Deserializer<'de>>::Error>
    where
        D: Deserializer<'de>,
    {
        Deserialize::deserialize(d).map(|i: Option<String>| {
            if let Some(t) = i {
                match t.to_uppercase().as_str() {
                    "DELETE" => Method::DELETE,
                    "GET" => Method::GET,
                    "PUT" => Method::PUT,
                    "POST" => Method::POST,
                    "PATCH" => Method::PATCH,
                    _ => panic!("Method unknown!"),
                }
            } else {
                panic!("No input value found during deserialization")
            }
        })
    }
}

#[derive(Deserialize, Serialize, Debug, PartialEq)]
/// Request struct
pub struct Request {
    /// Body of the request
    pub body: String,
    /// Method used, GET, PUT, POST, DELETE etc
    pub method: Method,
    /// Headers of the request
    pub headers: HashMap<String, String>,
    #[serde(deserialize_with = "url_deserialize")]
    #[serde(serialize_with = "url_serialize")]
    /// Request destination URL
    pub url: Url,
}

#[derive(Serialize, Debug, PartialEq)]
/// Response struct
pub struct Response {
    /// HttpStatusCode
    pub status: u16,
    /// Headers of the response
    pub headers: HashMap<String, String>,
    /// Body of the response
    pub body: String,
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
    use super::*;
    use rstest::*;
    use serde_json::from_str;
    use url::Url;

    #[test]
    fn request_serialize_ok() {
        let mut req = Request {
            body: "".to_string(),
            method: Method::GET,
            headers: Default::default(),
            url: Url::parse("https://test.se/").unwrap(),
        };
        req.headers
            .insert("content-type".to_string(), "application/json".to_string());

        let res = serde_json::to_string(&req).unwrap();

        assert_eq!(res, "{\"body\":\"\",\"method\":\"GET\",\"headers\":{\"content-type\":\"application/json\"},\"url\":\"https://test.se/\"}".to_string());
    }

    #[test]
    fn request_deserialize_ok() {
        let input = "{\"body\":\"\",\"method\":\"GET\",\"headers\":{\"content-type\":\"application/json\"},\"url\":\"https://test.se/\"}";
        let mut expected = Request {
            body: "".to_string(),
            method: Method::GET,
            headers: Default::default(),
            url: Url::parse("https://test.se/").unwrap(),
        };
        expected
            .headers
            .insert("content-type".to_string(), "application/json".to_string());

        let result: Request = from_str(input).unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn response_serialize_ok() {
        let mut req = Response {
            body: "Body".to_string(),
            headers: Default::default(),
            status: 200,
        };
        req.headers
            .insert("content-type".to_string(), "application/json".to_string());

        let res = serde_json::to_string(&req).unwrap();
        assert_eq!(
            res,
            "{\"status\":200,\"headers\":{\"content-type\":\"application/json\"},\"body\":\"Body\"}"
                .to_string()
        );
    }

    #[rstest(input, expected,
        case(&Method::GET, "\"GET\""),
        case(&Method::DELETE, "\"DELETE\""),
        case(&Method::PUT, "\"PUT\""),
        case(&Method::PATCH, "\"PATCH\""),
        case(&Method::POST, "\"POST\""),
    )]
    fn serialize_enum_ok(input: &Method, expected: &str) {
        assert_eq!(serde_json::to_string(input).unwrap(), expected);
    }

    #[rstest(
        input,
        expected,
        case("\"GeT\"", Method::GET),
        case("\"DELETE\"", Method::DELETE),
        case("\"Put\"", Method::PUT),
        case("\"patcH\"", Method::PATCH),
        case("\"POst\"", Method::POST)
    )]
    fn deserialize_enum_ok(input: &str, expected: Method) {
        let res: Method = serde_json::from_str(&input).unwrap();
        assert_eq!(res, expected);
    }
}
