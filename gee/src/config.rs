use std::{collections::HashMap, net::SocketAddr};

/// `Config` is the global, immutable configuration used to construct and run the Gee server.
#[derive(Clone)]
pub struct Config {
    /// `address` is the IP address where the Gee server will serve content.
    pub address: SocketAddr,

    /// `root_dir` is a relative or absolute path on which all relative resource lookups will be based.
    pub root_dir: String,

    /// `static_routes` map paths on the server to directories of static assets to be served.
    pub static_routes: HashMap<String, String>,
}
