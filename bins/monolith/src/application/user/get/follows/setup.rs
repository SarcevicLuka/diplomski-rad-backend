use std::sync::Arc;
use actix_web::{web, web::get};
use infrastructure::db::Postgres;
use super::{
    domain::GetFollows, 
    infrastructure::PgRepository, 
    http::{
        http_get_following::handle_get_following,
        http_get_follows::handle_get_follows
    }
};

#[allow(dead_code)]
pub fn routes(
    postgres: Arc<Postgres>, 
    cfg: &mut web::ServiceConfig
) {
    let service = GetFollows {
        repository: PgRepository {
            pg_pool: postgres,
        },
    };
    
    cfg.app_data(web::Data::new(service));
    cfg.service(
        web::resource("/user/{user_id}/follows")
        .route(get().to(handle_get_follows::<
            GetFollows<PgRepository>
        >))
    );

    cfg.service(
        web::resource("/user/{user_id}/following")
        .route(get().to(handle_get_following::<
            GetFollows<PgRepository>
        >))
    );
}