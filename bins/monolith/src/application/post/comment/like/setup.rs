use std::sync::Arc;
use actix_web::{web, web::post};
use infrastructure::db::Postgres;
use super::{
    domain::LikeComment, 
    infrastructure::PgRepository, 
    http::{
        http_like::handle_like_comment,
        http_remove_like::handle_remove_like_comment
    }
};

#[allow(dead_code)]
pub fn routes(
    postgres: Arc<Postgres>, 
    cfg: &mut web::ServiceConfig
) {
    let service = LikeComment {
        repository: PgRepository {
            pg_pool: postgres,
        },
    };
    
    cfg.app_data(web::Data::new(service));
    cfg.service(
        web::resource("/user/{user_id}/posts/{post_id}/comments/{comment_id}/like")
        .route(post().to(handle_like_comment::<
            LikeComment<PgRepository>
        >))
        .wrap(crate::middleware::AuthLogin)
    );

    cfg.service(
        web::resource("/user/{user_id}/posts/{post_id}/comments/{comment_id}/remove-like")
        .route(post().to(handle_remove_like_comment::<
            LikeComment<PgRepository>
        >))
        .wrap(crate::middleware::AuthLogin)
    );
}