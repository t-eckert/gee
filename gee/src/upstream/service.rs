use log::{debug, info};
use std::{
    future,
    task::{Context, Poll},
};

use hyper::{service::Service as HyperService, Body, Request, Response};

/// Service handles the requests received by Gee, routing them to Python or serving static files back to the caller.
pub struct Service {
    /// `root_dir` is the absolute path to the directory where Gee is running.
    pub root_dir: String,

    /// `static_dir` is the relative path to static content being served by Gee.
    pub static_dir: String,
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
        let body = Body::from(format!(
            "Root: {}\tStatic: {}\n{:#?}",
            self.root_dir, self.static_dir, req
        ));
        let rsp = rsp.status(200).body(body).unwrap();
        future::ready(Ok(rsp))
    }
}

/*
async fn process_request(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    info!("{} request received at {}", req.method(), req.uri());
    debug!("{:#?}", req);

    let environ = Environ::from_request(req);
    call(environ);

    let body = Body::empty();
    Ok(Response::new(body))
}

*/
