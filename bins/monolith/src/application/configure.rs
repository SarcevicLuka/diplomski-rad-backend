use actix_web::web;
use infrastructure::db::Postgres;
use std::sync::Arc;

pub fn configure(
    postgres: Arc<Postgres>, 
    cfg: &mut web::ServiceConfig
) {
    auth(postgres.clone(), cfg);
}

fn auth(
    postgres: Arc<Postgres>, 
    cfg: &mut web::ServiceConfig
) {
    crate::application::auth::authentication::setup::routes(postgres.clone(), cfg);
    crate::application::auth::registration::setup::routes(postgres, cfg)
}