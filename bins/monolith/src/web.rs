use std::sync::Arc;
use actix_cors::Cors;
use actix_web::{App, HttpServer, http};
use infrastructure::db::Postgres;

pub async fn setup_server(postgres: Arc<Postgres>) -> std::io::Result<()> {
    let address = format!(
        "{}:{}",
        config::get_default("LISTEN_ADDRESS", "127.0.0.1"),
        config::get_default("LISTEN_PORT", "8080")
    );

    let workers: usize = config::get_default("HTTP_SERVER_NUMBER_OF_WORKERS", "9")
        .parse()
        .unwrap_or(9);

    HttpServer::new(move|| {
        App::new()
            // Initialize global state
            .app_data(infrastructure::state::initialize())
            .wrap(setup_cors())
            .configure(|cfg| crate::application::configure(Arc::clone(&postgres), cfg))
    })
    .workers(workers)
    .bind(address)?
    .run()
    .await
}
fn setup_cors() -> actix_cors::Cors {
    Cors::default()
        .supports_credentials()
        .allowed_origin("http://localhost:5173")
        .allowed_headers(vec![
            http::header::ACCEPT,
            http::header::ALLOW,
            http::header::CONTENT_TYPE,
            http::header::AUTHORIZATION,
        ])
        .allowed_methods(vec!["POST", "GET", "PATCH", "DELETE"])
        .expose_headers(vec!["Authorization"])
        .max_age(3600)
}