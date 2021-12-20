extern crate pretty_env_logger;
#[macro_use]
extern crate log;

mod application;
mod environ;
mod server;

use std::net::SocketAddr;

use server::start;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    let socket_address = SocketAddr::from(([127, 0, 0, 1], 8080));

    start(socket_address).await;
}
