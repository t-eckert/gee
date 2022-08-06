extern crate log;
extern crate pretty_env_logger;

#[macro_use]

mod cli;
mod config;
mod macros;


#[tokio::main]
async fn main() {
    pretty_env_logger::init();
}
