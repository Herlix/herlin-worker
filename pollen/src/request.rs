use crate::body::Body;
use actix_router::{Path, Url};
use http::header::HeaderName;
use http::{HeaderMap, HeaderValue, Method, Uri};
use pollen_keyvault::JsValue;
use serde::Deserialize;
use std::collections::HashMap;
use std::convert::TryFrom;

impl From<JsValue> for HttpRequest {
    fn from(req: JsValue) -> Self {
        let res: Result<HttpRequest, JsValue> = req
            .into_serde()
            .map_err(|e| JsValue::from_str(e.to_string().as_str()))
            .map(|e: HttpRequestDef| e.into());
        res.unwrap()
    }
}

#[derive(Debug)]
pub struct HttpRequest {
    pub method: Method,
    pub headers: HeaderMap,
    pub path: Path<Url>,
    pub body: Body,
}

#[derive(Debug)]
struct MethodDef(Method);

#[derive(Debug)]
pub(crate) struct HeaderMapDef(HeaderMap);

#[derive(Debug)]
struct PathDef(Path<Url>);

/// Request coming in from JavaScript
#[derive(Debug, Deserialize)]
pub struct HttpRequestDef {
    method: MethodDef,
    headers: HeaderMapDef,
    path: PathDef,
    body: Body,
}

impl From<HttpRequestDef> for HttpRequest {
    fn from(def: HttpRequestDef) -> Self {
        HttpRequest {
            method: def.method.0,
            headers: def.headers.0,
            path: def.path.0,
            body: def.body,
        }
    }
}

impl<'de> Deserialize<'de> for MethodDef {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Deserialize::deserialize(deserializer)
            .map(|i: Option<String>| MethodDef(Method::try_from(i.unwrap().as_str()).unwrap()))
    }
}

impl<'de> Deserialize<'de> for HeaderMapDef {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Deserialize::deserialize(deserializer).map(|i: Option<HashMap<String, String>>| {
            let map = i.unwrap();
            let mut result = HeaderMap::with_capacity(map.keys().len());
            map.iter().for_each(|(hn, hv)| {
                let header_name = HeaderName::try_from(hn).expect("Unknown HeaderName");
                let header_value = HeaderValue::try_from(hv).expect("Unknown HeaderValue");
                result.insert(header_name, header_value);
            });
            HeaderMapDef(result)
        })
    }
}

impl<'de> Deserialize<'de> for PathDef {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Deserialize::deserialize(deserializer).map(|i: Option<String>| {
            PathDef(Path::new(Url::new(i.unwrap().parse::<Uri>().unwrap())))
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use http::HeaderValue;

    #[test]
    fn deserialize_empty() {
        let input = "";
        let result: Result<HttpRequestDef, serde_json::error::Error> = serde_json::from_str(&input);
        assert!(result.is_err());
    }

    #[test]
    fn deserialize_random_header() {
        let input = r#"{
            "method": "PUT",
            "headers": {
                "what": "Nothing"
            },
            "path": "https://examples.com/",
            "body": ""
            }"#;
        let result: Result<HttpRequestDef, serde_json::error::Error> = serde_json::from_str(&input);
        assert!(result.is_ok());
        let result = result.unwrap();
        assert_eq!(&result.headers.0.len(), &1);
        assert!(&result.headers.0.get("what").unwrap().eq("Nothing"))
    }

    #[test]
    fn deserialize_valid() {
        let input = r#"{
            "method": "DELETE",
            "headers": {
                "accept": "text/html,application/xhtml+xml,application/xml;q=0.9,image/webp,*/*;q=0.8"
            },
            "path": "https://examples.com/",
            "body": "Something"
            }"#;

        let mut headers = HeaderMap::with_capacity(1);
        headers.insert(
            http::header::ACCEPT,
            HeaderValue::try_from(
                "text/html,application/xhtml+xml,application/xml;q=0.9,image/webp,*/*;q=0.8",
            )
            .unwrap(),
        );

        let expected = HttpRequestDef {
            method: MethodDef(Method::DELETE),
            headers: HeaderMapDef(headers),
            path: PathDef(Path::new(Url::new(
                "https://examples.com/".parse::<Uri>().unwrap(),
            ))),
            body: Body::Message("Something".to_string()),
        };
        let result: HttpRequestDef = serde_json::from_str(input).unwrap();
        assert_eq!(result.path.0.path(), expected.path.0.path());
        assert_eq!(result.body, expected.body);
        assert_eq!(result.method.0, expected.method.0);
        assert_eq!(result.headers.0.len(), expected.headers.0.len());
        assert_eq!(format!("{:?}", &result), format!("{:?}", expected));
    }

    #[test]
    fn into() {
        let mut headers = HeaderMap::with_capacity(1);
        headers.insert(
            http::header::ACCEPT,
            HeaderValue::try_from(
                "text/html,application/xhtml+xml,application/xml;q=0.9,image/webp,*/*;q=0.8",
            )
            .unwrap(),
        );

        let input = HttpRequestDef {
            method: MethodDef(Method::GET),
            headers: HeaderMapDef(headers),
            path: PathDef(Path::new(Url::new(
                "https://example.com/".parse::<Uri>().unwrap(),
            ))),
            body: Body::Message("hej".to_string()),
        };

        let result: HttpRequest = input.into();
        assert_eq!(format!("{:?}", result), "HttpRequest { method: GET, headers: {\"accept\": \"text/html,application/xhtml+xml,application/xml;q=0.9,image/webp,*/*;q=0.8\"}, path: Path { path: Url { uri: https://example.com/, path: None }, skip: 0, segments: [] }, body: Message(\"hej\") }".to_string());
    }

    #[test]
    fn from() {
        let mut headers = HeaderMap::with_capacity(1);
        headers.insert(
            http::header::ACCEPT,
            HeaderValue::try_from(
                "text/html,application/xhtml+xml,application/xml;q=0.9,image/webp,*/*;q=0.8",
            )
            .unwrap(),
        );

        let input = HttpRequestDef {
            method: MethodDef(Method::POST),
            headers: HeaderMapDef(headers),
            path: PathDef(Path::new(Url::new(
                "https://exa.com/?hej=nej".parse::<Uri>().unwrap(),
            ))),
            body: Body::Empty,
        };

        let result = HttpRequest::from(input);
        assert_eq!(format!("{:?}", result), "HttpRequest { method: POST, headers: {\"accept\": \"text/html,application/xhtml+xml,application/xml;q=0.9,image/webp,*/*;q=0.8\"}, path: Path { path: Url { uri: https://exa.com/?hej=nej, path: None }, skip: 0, segments: [] }, body: Empty }".to_string());
    }
}
