use std::sync::Arc;
use actix_web::{web, web::get};
use infrastructure::db::Postgres;
use super::{
    domain::GetPost, 
    infrastructure::PgRepository, 
    http::handle_get_users_posts
};

#[allow(dead_code)]
pub fn routes(
    postgres: Arc<Postgres>, 
    cfg: &mut web::ServiceConfig
) {
    let service = GetPost {
        repository: PgRepository {
            pg_pool: postgres,
        },
    };
    
    cfg.app_data(web::Data::new(service));
    cfg.service(
        web::resource("/user/{user_id}/posts")
        .route(get().to(handle_get_users_posts::<
            GetPost<PgRepository>
        >))
    );
}