extern crate log;
extern crate pretty_env_logger;
#[macro_use]

mod config;
mod downstream;
mod environ;
mod upstream;

use std::{env, net::SocketAddr};

use config::Config;
use upstream::Server;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    let args: Vec<String> = env::args().collect();
    let default_root_dir = ".".to_owned();
    let root_dir = args.get(1).unwrap_or(&default_root_dir).to_owned();
    let address = SocketAddr::from(([127, 0, 0, 1], 8080));

    let config = Config {
        address,
        root_dir,
        static_dir: "/static".to_owned(),
    };

    Server { config }.start().await.unwrap();
}
