#[warn(missing_debug_implementations, rust_2018_idioms, missing_docs)]
use actix_router::{Path, Url};
use http::Uri;
use request::HttpRequest;
use response::HttpResponse;
use std::future::Future;

pub use pollen_keyvault::{CloudFlareKV, JSError};
pub use pollen_keyvault_derive::*;

/// All body related logic
pub mod body;
/// All Error related logic
pub mod errors;
/// Request related logic
pub mod request;
/// Response related logic
pub mod response;

struct Route<F>
where
    F: Future<Output = HttpResponse> + 'static,
{
    pub path: Path<Url>,
    pub responder: Box<dyn Fn(HttpRequest) -> F>,
}

/// Main app builder, register routes and get response
pub struct App<F>
where
    F: Future<Output = HttpResponse> + 'static,
{
    request: HttpRequest,
    routes: Vec<Route<F>>,
}

impl<'req, F> App<F>
where
    F: Future<Output = HttpResponse> + 'static,
{
    pub fn new(request: HttpRequest) -> Self {
        App {
            request,
            routes: Vec::default(),
        }
    }

    pub fn reg(&mut self, route: &str, res: impl Fn(HttpRequest) -> F + 'static) -> &mut Self {
        &self.routes.push(Route {
            path: Path::new(Url::new(route.parse::<Uri>().unwrap())),
            responder: Box::new(res),
        });
        self
    }

    pub fn response(self) -> impl Future<Output = HttpResponse> {
        // TODO: Get corerect route
        let m: &Route<F> = self.routes.get(0).unwrap();
        m.responder.as_ref()(self.request)
    }
}
