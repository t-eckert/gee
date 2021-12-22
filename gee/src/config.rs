use std::net::SocketAddr;

#[derive(Clone)]
pub struct Config {
    pub address: SocketAddr,
    pub root_dir: String,
    pub static_dir: String,
}
