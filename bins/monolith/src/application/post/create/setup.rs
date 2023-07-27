use std::sync::Arc;
use actix_web::{web, web::post};
use infrastructure::db::Postgres;
use super::{
    domain::CreatePost, 
    infrastructure::PgRepository, http::handle_create_post
};

#[allow(dead_code)]
pub fn routes(
    postgres: Arc<Postgres>, 
    cfg: &mut web::ServiceConfig
) {
    let service = CreatePost {
        repository: PgRepository {
            pg_pool: postgres,
        },
    };
    
    cfg.app_data(web::Data::new(service));
    cfg.service(
        web::resource("/create-post")
        .route(post().to(handle_create_post::<
            CreatePost<PgRepository>
        >))
        .wrap(crate::middleware::AuthLogin)
    );
}