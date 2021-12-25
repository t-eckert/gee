use hyper::Server as HyperServer;
use log::info;

use super::service_builder::ServiceBuilder;
use crate::config::Config;

/// Server is a wrapper around a `hyper::Server` that allows configuration of the Gee server.
pub struct Server {
    /// `config` is the global immutable configuration for the Gee server used to properly construct the server and the
    /// processes it spawns.
    pub config: Config,
}

impl Server {
    pub async fn start(&self) -> Result<(), Box<dyn std::error::Error>> {
        let server = HyperServer::bind(&self.config.address).serve(ServiceBuilder {
            config: self.config.clone(),
        });
        server.await?;
        info!("Gee server running at {}", self.config.address);
        Ok(())
    }
}
