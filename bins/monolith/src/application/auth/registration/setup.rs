use actix_web::web;
use std::sync::Arc;
use infrastructure::db::{Postgres};
use super::{
    domain::Registration, 
    http::handle_register,
    infrastructure::{
        PgRepository, PgService
    }
};

pub fn routes(
    postgres: Arc<Postgres>,
    cfg: &mut web::ServiceConfig
) {
    let service = Registration {
        repository: PgRepository {
            pg_pool: postgres.clone(),
        },
        service: PgService {
            pg_pool: postgres,
        },
    };

    cfg.app_data(web::Data::new(service));
    cfg.service(
        web::resource("/auth/register")
        .route(web::post().to(handle_register::<
            Registration<PgRepository, PgService>    
        >)));
}