use std::sync::Arc;
use actix_web::{web, web::post};
use infrastructure::db::Postgres;
use super::{
    domain::FollowUser, 
    infrastructure::PgRepository, 
    http::{
        http_follow::handle_follow_user,
        http_unfollow::handle_unfollow_user
    }
};

#[allow(dead_code)]
pub fn routes(
    postgres: Arc<Postgres>, 
    cfg: &mut web::ServiceConfig
) {
    let service = FollowUser {
        repository: PgRepository {
            pg_pool: postgres,
        },
    };
    
    cfg.app_data(web::Data::new(service));
    cfg.service(
        web::resource("/user/{user_id}/follow")
        .route(post().to(handle_follow_user::<
            FollowUser<PgRepository>
        >))
        .wrap(crate::middleware::AuthLogin)
    );

    cfg.service(
        web::resource("/user/{user_id}/unfollow")
        .route(post().to(handle_unfollow_user::<
            FollowUser<PgRepository>
        >))
        .wrap(crate::middleware::AuthLogin)
    );
}