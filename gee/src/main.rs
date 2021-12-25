extern crate log;
extern crate pretty_env_logger;

mod config;
mod downstream;
mod environ;
#[macro_use]
mod macros;
mod upstream;

use std::{env, net::SocketAddr};

use config::Config;
use upstream::Server;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    let args: Vec<String> = env::args().collect();
    let config = build_config(args);

    Server { config }.start().await.unwrap();
}

fn build_config(args: Vec<String>) -> Config {
    let address = SocketAddr::from(([127, 0, 0, 1], 8080));
    let default_root_dir = ".".to_owned();
    let root_dir = args.get(1).unwrap_or(&default_root_dir).to_owned();

    let static_routes = hashmap!["/static".to_owned() => "./static/".to_owned()];

    Config {
        address,
        root_dir,
        static_routes,
    }
}
