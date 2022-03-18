use crate::handlers::static_service_handler;
use hyper::{service::Service as HyperService, Body, Request, Response};
use log::{debug, info};
use std::{
    future::{self, Future},
    task::{Context, Poll},
};

use crate::config::Config;

/// `Service` handles the requests received by Gee, routing them to the correct
/// handler based on the request path. These handlers could be static file
/// handlers or Python handlers which pass the request to a callable.
pub struct Service {
    /// `config` is the global, immutable configuration used to construct and
    /// run the Gee server.
    config: Config,
}

impl Service {
    fn get_handler(
        &self,
        path: &str,
    ) -> Option<impl Fn(Request<Body>, Config) -> Future<Response<Body>>> {
        if self.config.is_static_path(path) {
            Some(static_service_handler)
        }

        None
    }

    /// `resolve_static_path` receives the `path` from the URI (e.g. /static/hello.txt) and checks it against the
    /// `static_routes` defined on the service. These `static_routes` map URI paths to UNIX-like paths (e.g.
    /// /static => ./static/). If there exists a key in `static_routes` which begins with the same characters
    /// as the `path`, the key will be stripped from the beginning of the `path` and replaced with corresponding
    /// value so that the server can look up the file and serve it to the user. If the resulting `path` is a directory,
    /// `index.html` will be appended to the path so that the default web page may be served.
    fn resolve_static_path(&self, path: &str) -> Option<String> {
        let matching_route = self
            .static_routes
            .iter()
            .filter(|(server_path, _)| path.starts_with(*server_path))
            .next();

        let static_route = match matching_route {
            Some(static_route) => static_route,
            None => return None,
        };

        let mut static_path = static_route.1.clone();
        static_path.push_str(&path[static_route.0.len()..path.len()]);

        if static_path.chars().last().unwrap() == '/' {
            static_path.push_str("index.html")
        }

        Some(static_path)
    }
}

impl HyperService<Request<Body>> for Service {
    type Response = Response<Body>;
    type Error = hyper::Error;
    type Future = future::Ready<Result<Self::Response, Self::Error>>;

    fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Ok(()).into()
    }

    /// `call` receives a request from the caller and routes it to the correct
    /// handler then returns the response to the caller.
    fn call(&mut self, req: Request<Body>) -> Self::Future {
        info!("{} request received at {}", req.method(), req.uri());
        debug!("{:#?}", req);

        let handler = self.get_handler(&req.uri().path());
        let response = handler(req, self.config.clone());

        future::ready(Ok(response))
    }
}

#[cfg(test)]
mod test {
    use crate::config::Config;

    use super::*;
}
