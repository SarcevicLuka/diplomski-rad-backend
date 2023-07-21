use std::sync::Arc;
use infrastructure::db::{Postgres};

#[allow(unused_imports)]
#[macro_use]
extern crate log;

mod web;
mod application;
mod middleware;

#[actix_web::main]
async fn main() {
    support::helpers::setup_logger();
    config::initialize();
    debug!("Configuration loaded and ready");

    debug!("Monolith application starting");
    
    let postgres = Arc::new(Postgres::new());

    web::setup_server(postgres).await.unwrap();
}