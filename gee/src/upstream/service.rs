use crate::{
    downstream::{call_application, serve_file},
    environ::Environ,
};
use hyper::{service::Service as HyperService, Body, Request, Response};
use log::{debug, info};
use std::{
    collections::HashMap,
    future,
    task::{Context, Poll},
};

/// `Service` handles the requests received by Gee, routing them to Python or serving static files back to the caller.
pub struct Service {
    /// `root_dir` is the absolute path to the directory where Gee is running.
    pub root_dir: String,

    // `static_routes` maps routes on the server to directories of static assets and serves the content at those routes.
    pub static_routes: HashMap<String, String>,
}

impl Service {
    /// `is_static_request` checks the path of the request against the `static_dir` of the `Service` and returns true
    /// if the request path is a child of the `static_dir` and is therefore a request for a static file/asset. This
    /// does not check if the file being requested exists.
    fn is_static_request(&self, path: &str) -> bool {
        self.static_routes
            .iter()
            .any(|(server_path, _)| path.starts_with(server_path))
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

    /// `call` receives a request from the user and routes it to the `serve_file` function if the request is for a
    /// static asset, otherwise an `Environ` object is built from the request and passed to the `call_application`
    /// function which will execute the request against the Python web application according to the WSGI spec.
    fn call(&mut self, req: Request<Body>) -> Self::Future {
        info!("{} request received at {}", req.method(), req.uri());
        debug!("{:#?}", req);

        let request_result = if self.is_static_request(req.uri().path()) {
            let static_path = self
                .resolve_static_path(req.uri().path())
                .expect("Cannot resolve static path");
            serve_file(&static_path)
        } else {
            let environ = Environ::from_request(&req);
            call_application(environ)
        };

        let rsp = Response::builder();
        let response = match request_result {
            Some(content) => rsp.status(200).body(Body::from(content)).unwrap(),
            None => rsp.status(404).body(Body::from(vec![])).unwrap(),
        };

        future::ready(Ok(response))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_is_static_request() {
        #[derive(Debug, Clone)]
        struct Case {
            pub root_dir: String,
            pub static_routes: HashMap<String, String>,
            pub path: String,
            pub expected: bool,
        }

        let cases = vec![
            Case {
                root_dir: "/".to_owned(),
                static_routes: hashmap!["/static".to_owned() => "./static".to_owned()],
                path: "/static".to_owned(),
                expected: true,
            },
            Case {
                root_dir: "/".to_owned(),
                static_routes: hashmap!["/static".to_owned() => "./static".to_owned()],
                path: "/static/file.json".to_owned(),
                expected: true,
            },
            Case {
                root_dir: "/".to_owned(),
                static_routes: hashmap!["/static".to_owned() => "./static".to_owned()],
                path: "/".to_owned(),
                expected: false,
            },
        ];

        for case in cases {
            let service = Service {
                root_dir: case.root_dir.clone(),
                static_routes: case.static_routes.clone(),
            };

            let actual = service.is_static_request(&case.path);

            assert_eq!(case.expected, actual, "{:#?}", case);
        }
    }
}
