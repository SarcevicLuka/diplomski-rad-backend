use actix_web::web;
use infrastructure::db::Postgres;
use std::sync::Arc;

pub fn configure(
    postgres: Arc<Postgres>, 
    cfg: &mut web::ServiceConfig
) {
    auth(postgres.clone(), cfg);
    user(postgres.clone(), cfg);
    post(postgres, cfg);
}

fn auth(
    postgres: Arc<Postgres>, 
    cfg: &mut web::ServiceConfig
) {
    crate::application::auth::authentication::setup::routes(postgres.clone(), cfg);
    crate::application::auth::registration::setup::routes(postgres, cfg);
}

fn user(
    postgres: Arc<Postgres>,
    cfg: &mut web::ServiceConfig
) {
    crate::application::user::follow::setup::routes(postgres.clone(), cfg);
    crate::application::user::get::info::setup::routes(postgres.clone(), cfg);
    crate::application::user::get::posts::setup::routes(postgres, cfg);
}

fn post(
    postgres: Arc<Postgres>,
    cfg: &mut web::ServiceConfig
) {
    crate::application::post::create::setup::routes(postgres.clone(), cfg);
    crate::application::post::delete::setup::routes(postgres.clone(), cfg);
    crate::application::post::edit::setup::routes(postgres.clone(), cfg);
    crate::application::post::get::setup::routes(postgres.clone(), cfg);
    crate::application::post::like::setup::routes(postgres.clone(), cfg);

    crate::application::post::comment::create::setup::routes(postgres.clone(), cfg);
    crate::application::post::comment::delete::setup::routes(postgres.clone(), cfg);
    crate::application::post::comment::edit::setup::routes(postgres.clone(), cfg);
    crate::application::post::comment::get::setup::routes(postgres.clone(), cfg);
    crate::application::post::comment::like::setup::routes(postgres, cfg);
}