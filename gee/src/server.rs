use hyper::{
    service::{make_service_fn, service_fn},
    Body, Request, Response, Server,
};

use crate::application::call;
use crate::environ::Environ;
use std::{convert::Infallible, net::SocketAddr};

pub async fn start(socket_address: SocketAddr) {
    let make_svc =
        make_service_fn(|_conn| async { Ok::<_, Infallible>(service_fn(process_request)) });

    let server = Server::bind(&socket_address).serve(make_svc);

    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}

async fn process_request(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    info!("{} request received at {}", req.method(), req.uri());
    println!("{:#?}", req);

    let environ = Environ::from_request(req);
    call(environ);

    let body = Body::empty();
    Ok(Response::new(body))
}
