use crate::body::Body;
use crate::errors::{HttpError, ResponseError};
use http::header::HeaderMap;
use http::{HeaderValue, StatusCode};
use serde::ser::{Serialize, SerializeStruct, Serializer};
use std::collections::HashMap;
use std::convert::TryFrom;

const DEFAULT_HEADER_CAPACITY: usize = 12;

pub struct HttpResponse {
    status: StatusCode,
    headers: HeaderMap,
    body: Body,
    error: Option<HttpError>,
}

impl HttpResponse {
    pub fn new(status: StatusCode) -> Self {
        HttpResponse {
            status,
            headers: HeaderMap::with_capacity(DEFAULT_HEADER_CAPACITY),
            body: Body::Empty,
            error: None,
        }
    }

    pub fn content_type<V>(&mut self, value: V) -> &mut Self
    where
        HeaderValue: TryFrom<V>,
        <HeaderValue as TryFrom<V>>::Error: Into<HttpError>,
    {
        if self.error.is_none() {
            match HeaderValue::try_from(value) {
                Ok(value) => {
                    self.headers.insert(http::header::CONTENT_TYPE, value);
                }
                Err(e) => self.error = Some(e.into()),
            }
        }
        self
    }

    pub fn response(&mut self, body: Body) -> HttpResponse {
        if let Some(e) = self.error.take() {
            e.error_response()
        } else {
            HttpResponse {
                status: self.status,
                headers: self.headers.clone(),
                body: body,
                error: None,
            }
        }
    }
    /// Get a mutable reference to the headers
    pub fn headers_mut(&mut self) -> &mut HeaderMap {
        &mut self.headers
    }

    /// Get a mutable reference to the body
    pub fn set_body(&mut self, body: Body) {
        self.body = body;
    }
}

impl Default for HttpResponse {
    fn default() -> Self {
        HttpResponse {
            status: StatusCode::OK,
            headers: HeaderMap::with_capacity(DEFAULT_HEADER_CAPACITY),
            body: Body::None,
            error: None,
        }
    }
}

#[allow(unused_must_use)]
impl Serialize for HttpResponse {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
    where
        S: Serializer,
    {
        let mut headers = HashMap::with_capacity(self.headers.len());
        for (key, value) in &self.headers {
            headers.insert(key.as_str(), value.to_str().unwrap());
        }

        let mut state = serializer.serialize_struct("Response", 3)?;
        state.serialize_field("status", &self.status.as_str());
        state.serialize_field("body", &self.body);
        state.serialize_field("headers", &headers);
        state.end()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn serialize_simple() {
        let mut headers = HeaderMap::with_capacity(1);
        headers.insert(
            http::header::ACCEPT,
            HeaderValue::try_from(
                "text/html,application/xhtml+xml,application/xml;q=0.9,image/webp,*/*;q=0.8",
            )
            .unwrap(),
        );

        let input = HttpResponse {
            status: StatusCode::OK,
            headers,
            body: Body::Message("Woop Woop".to_string()),
            error: None,
        };
        let result: Result<String, serde_json::error::Error> = serde_json::to_string(&input);
        assert!(result.is_ok());
        let result = result.unwrap();
        assert_eq!(result, "{\"status\":\"200\",\"body\":\"Woop Woop\",\"headers\":{\"accept\":\"text/html,application/xhtml+xml,application/xml;q=0.9,image/webp,*/*;q=0.8\"}}".to_string());
    }

    #[test]
    fn serialize_empty_headers() {
        let input = HttpResponse {
            status: StatusCode::OK,
            headers: HeaderMap::default(),
            body: Body::Message("Woop Woop".to_string()),
            error: None,
        };
        let result: Result<String, serde_json::error::Error> = serde_json::to_string(&input);
        assert!(result.is_ok());
        let result = result.unwrap();
        assert_eq!(
            result,
            "{\"status\":\"200\",\"body\":\"Woop Woop\",\"headers\":{}}".to_string()
        );
    }
}
