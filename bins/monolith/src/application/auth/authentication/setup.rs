use actix_web::web::{self, post};
use std::sync::Arc;
use infrastructure::db::{Postgres};
use super::{domain::Login, infrastructure::PgRepository, http::handle_login};

pub fn routes(
    postgres: Arc<Postgres>,
    cfg: &mut web::ServiceConfig
) {
    let service = Login {
        repository: PgRepository {
            pg_pool: postgres,
        }
    };

    cfg.app_data(web::Data::new(service));
    cfg.service(
        web::resource("/auth/login")
        .route(post().to(handle_login::<
            Login<PgRepository>    
        >)));
}