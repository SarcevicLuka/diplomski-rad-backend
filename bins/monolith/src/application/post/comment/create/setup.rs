use std::sync::Arc;
use actix_web::{web, web::post};
use infrastructure::db::Postgres;
use super::{
    domain::CreateComment, 
    infrastructure::PgRepository, http::handle_create_comment
};

#[allow(dead_code)]
pub fn routes(
    postgres: Arc<Postgres>, 
    cfg: &mut web::ServiceConfig
) {
    let service = CreateComment {
        repository: PgRepository {
            pg_pool: postgres,
        },
    };
    
    cfg.app_data(web::Data::new(service));
    cfg.service(
        web::resource("/posts/{post_id}/comments/create")
        .route(post().to(handle_create_comment::<
            CreateComment<PgRepository>
        >))
        .wrap(crate::middleware::AuthLogin)
    );
}