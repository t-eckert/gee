use log::{debug, info};
use std::{
    future,
    task::{Context, Poll},
};

use hyper::{service::Service as HyperService, Body, Request, Response};

use crate::{
    downstream::{call_application, serve_file},
    environ::Environ,
};

/// Service handles the requests received by Gee, routing them to Python or serving static files back to the caller.
pub struct Service {
    /// `root_dir` is the absolute path to the directory where Gee is running.
    pub root_dir: String,

    /// `static_dir` is the relative path to static content being served by Gee.
    pub static_dir: String,
}

impl Service {
    /// `is_static_request` checks the path of the request against the `static_dir` of the `Service` and returns true
    /// if the request path is a child of the `static_dir` and is therefore a request for a static file/asset. This
    /// does not check if the file being requested exists.
    fn is_static_request(&self, path: &str) -> bool {
        let static_dir_length = self.static_dir.len();

        if path.len() < static_dir_length {
            return false;
        }

        self.static_dir == path[0..static_dir_length]
    }
}

impl HyperService<Request<Body>> for Service {
    type Response = Response<Body>;
    type Error = hyper::Error;
    type Future = future::Ready<Result<Self::Response, Self::Error>>;

    fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Ok(()).into()
    }

    fn call(&mut self, req: Request<Body>) -> Self::Future {
        let rsp = Response::builder();

        info!("{} request received at {}", req.method(), req.uri());
        debug!("{:#?}", req);

        let environ = Environ::from_request(&req);
        let content = if self.is_static_request(req.uri().path()) {
            serve_file(req.uri().path())
        } else {
            call_application(environ)
        };

        let body = Body::from(content.unwrap_or(vec![]));
        let rsp = rsp.status(200).body(body).unwrap();
        future::ready(Ok(rsp))
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
            pub static_dir: String,
            pub path: String,
            pub expected: bool,
        }

        let cases = vec![
            Case {
                root_dir: "/".to_owned(),
                static_dir: "/static".to_owned(),
                path: "/static".to_owned(),
                expected: true,
            },
            Case {
                root_dir: "/".to_owned(),
                static_dir: "/static".to_owned(),
                path: "/static/file.json".to_owned(),
                expected: true,
            },
            Case {
                root_dir: "/".to_owned(),
                static_dir: "/static".to_owned(),
                path: "/".to_owned(),
                expected: false,
            },
        ];

        for case in cases {
            let service = Service {
                root_dir: case.root_dir.clone(),
                static_dir: case.static_dir.clone(),
            };

            let actual = service.is_static_request(&case.path);

            assert_eq!(case.expected, actual, "{:#?}", case);
        }
    }
}
