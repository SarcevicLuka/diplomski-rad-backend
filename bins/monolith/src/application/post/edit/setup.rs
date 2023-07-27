use std::sync::Arc;
use actix_web::{web, web::post};
use infrastructure::db::Postgres;
use super::{
    domain::EditPost, 
    infrastructure::PgRepository, http::handle_edit_post
};

#[allow(dead_code)]
pub fn routes(
    postgres: Arc<Postgres>, 
    cfg: &mut web::ServiceConfig
) {
    let service = EditPost {
        repository: PgRepository {
            pg_pool: postgres,
        },
    };
    
    cfg.app_data(web::Data::new(service));
    cfg.service(
        web::resource("/user/{user_id}/posts/{post_id}/edit")
        .route(post().to(handle_edit_post::<
            EditPost<PgRepository>
        >))
        .wrap(crate::middleware::AuthLogin)
    );
}