use std::net::SocketAddr;

use hyper::Server as HyperServer;
use log::info;

use super::service_builder::ServiceBuilder;
use crate::config::Config;

/// Server is a wrapper around a `hyper::Server` that allows configuration of
/// the Gee server.
pub struct Server {
    /// `config` is the global immutable configuration for the Gee server used
    /// to properly construct the server and the processes it spawns.
    config: Config,

    /// `server` is the `hyper::Server` that will be used to serve requests.
    server: HyperServer<I, S>,
}

impl Server {
    /// `new` creates a new `Server` instance using a config object.
    pub fn new(config: Config) -> Self {
        let address = SocketAddr::new(config.address, config.port);

        let server = HyperServer::bind(&address).serve(ServiceBuilder {
            config: config.clone(),
        });

        Self { config, server }
    }

    /// `start` starts the server.
    pub async fn start(&self) -> Result<(), Box<dyn std::error::Error>> {
        if self.config.application.is_some() && self.config.application_name.is_some() {
            pyo3::prepare_freethreaded_python();
        }

        self.server.await?;

        info!("Gee server running at {}", self.config.socket_address());
        Ok(())
    }
}
