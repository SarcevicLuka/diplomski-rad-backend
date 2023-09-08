use std::sync::Arc;
use actix_web::{web, web::get};
use infrastructure::db::Postgres;
use super::{
    domain::SearchUsers, 
    infrastructure::PgRepository, 
    http::handle_get_searched_users
};

#[allow(dead_code)]
pub fn routes(
    postgres: Arc<Postgres>, 
    cfg: &mut web::ServiceConfig
) {
    let service = SearchUsers {
        repository: PgRepository {
            pg_pool: postgres,
        },
    };
    
    cfg.app_data(web::Data::new(service));
    cfg.service(
        web::resource("/search/users")
        .route(get().to(handle_get_searched_users::<
            SearchUsers<PgRepository>
        >))
    );
}