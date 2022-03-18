use std::{
    future,
    task::{Context, Poll},
};

use hyper::service::Service as HyperService;

use super::service::Service;
use crate::Config;

/// `ServiceBuilder` creates a new instance of `Service` based on given config.
pub struct ServiceBuilder {
    pub config: Config,
}

impl<T> HyperService<T> for ServiceBuilder {
    type Response = Service;
    type Error = std::io::Error;
    type Future = future::Ready<Result<Self::Response, Self::Error>>;

    fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Ok(()).into()
    }

    fn call(&mut self, _: T) -> Self::Future {
        future::ready(Ok(Service {
            config: self.config.clone(),
        }))
    }
}
