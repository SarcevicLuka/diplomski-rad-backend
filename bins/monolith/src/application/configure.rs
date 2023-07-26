use actix_web::web;
use infrastructure::db::Postgres;
use std::sync::Arc;

pub fn configure(
    postgres: Arc<Postgres>, 
    cfg: &mut web::ServiceConfig
) {
    auth(postgres.clone(), cfg);
    user(postgres.clone(), cfg);
}

fn auth(
    postgres: Arc<Postgres>, 
    cfg: &mut web::ServiceConfig
) {
    crate::application::auth::authentication::setup::routes(postgres.clone(), cfg);
    crate::application::auth::registration::setup::routes(postgres.clone(), cfg);
}

fn user(
    postgres: Arc<Postgres>,
    cfg: &mut web::ServiceConfig
) {
    crate::application::user::get::setup::routes(postgres.clone(), cfg);

    crate::application::user::comment::create::setup::routes(postgres.clone(), cfg);
    crate::application::user::comment::edit::setup::routes(postgres.clone(), cfg);
    crate::application::user::comment::get::setup::routes(postgres.clone(), cfg);

    crate::application::user::post::create::setup::routes(postgres.clone(), cfg);
    crate::application::user::post::get::setup::routes(postgres.clone(), cfg);
}