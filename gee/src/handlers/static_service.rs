pub fn static_service_handler(req: Request<Body>) -> Self::Future<Response<Body>> {
    let path = req.uri().path();
    let static_path = self
        .resolve_static_path(path)
        .expect("Cannot resolve static path");
    serve_file(&static_path);

    let rsp = Response::builder();
    let response = match request_result {
        Some(content) => rsp.status(200).body(Body::from(content)).unwrap(),
        None => rsp.status(404).body(Body::from(vec![])).unwrap(),
    };
}
