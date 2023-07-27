use std::sync::Arc;
use actix_web::{web, web::post};
use infrastructure::db::Postgres;
use super::{
    domain::LikePost, 
    infrastructure::PgRepository, http::handle_like_post
};

#[allow(dead_code)]
pub fn routes(
    postgres: Arc<Postgres>, 
    cfg: &mut web::ServiceConfig
) {
    let service = LikePost {
        repository: PgRepository {
            pg_pool: postgres,
        },
    };
    
    cfg.app_data(web::Data::new(service));
    cfg.service(
        web::resource("/user/{user_id}/posts/{post_id}/like")
        .route(post().to(handle_like_post::<
            LikePost<PgRepository>
        >))
        .wrap(crate::middleware::AuthLogin)
    );
}