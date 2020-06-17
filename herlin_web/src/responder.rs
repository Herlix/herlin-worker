use crate::{body::Body, errors::Error, request::HttpRequest, response::HttpResponse};
use futures::future::ok;
use futures::future::Ready;
use http::StatusCode;
use std::future::Future;

/// Trait implemented by types that can be converted to http response.
///
/// Types that implement this trait can be used to return a type handler.
pub trait Responder {
    /// The response error which can be returned.
    type Error: Into<Error>;

    /// The future response value
    type Future: Future<Output = Result<HttpResponse, Self::Error>>;

    fn respond_to(self, req: &HttpRequest) -> Self::Future;
}

impl Responder for String {
    type Error = Error;
    type Future = Ready<Result<HttpResponse, Error>>;

    fn respond_to(self, _: &HttpRequest) -> Self::Future {
        ok(HttpResponse::new(StatusCode::OK)
            .content_type("text/plain; charset=utf-8")
            .response(Body::Message(self)))
    }
}
