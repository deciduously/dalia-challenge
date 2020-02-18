// main.rs
// Entry point - tokio

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;

use hyper::{
    service::{make_service_fn, service_fn},
    Server,
};
use log::info;
use std::convert::Infallible;

mod config;
mod db;
mod error;
mod handlers;
mod models;
mod router;
mod schema;
mod scrape;
mod templates;

// Re-exports for more convenient in-crate `use`

pub use config::*;
pub use db::*;
pub use error::*;
pub use handlers::*;
pub use models::*;
pub use router::*;
pub use schema::*;
pub use scrape::*;
pub use templates::*;

use config::{init_logging, OPT};
use router::router;

#[tokio::main]
async fn main() {
    init_logging(2).expect("Could not init logging"); // For now just INFO

    let addr = format!("{}:{}", OPT.address, OPT.port)
        .parse()
        .expect("Should parse net::SocketAddr");
    let make_svc = make_service_fn(|_conn| async { Ok::<_, Infallible>(service_fn(router)) });

    let server = Server::bind(&addr).serve(make_svc);

    info!("Serving {} on {}", env!("CARGO_PKG_NAME"), addr);

    if let Err(e) = server.await {
        eprintln!("Server error: {}", e);
    }
}
