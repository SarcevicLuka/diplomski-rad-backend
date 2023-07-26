use std::sync::Arc;
use actix_web::{web, web::get};
use infrastructure::db::Postgres;
use super::{
    domain::GetComments, 
    infrastructure::PgRepository, 
    http::handle_get_post_comments
};

#[allow(dead_code)]
pub fn routes(
    postgres: Arc<Postgres>, 
    cfg: &mut web::ServiceConfig
) {
    let service = GetComments {
        repository: PgRepository {
            pg_pool: postgres,
        },
    };
    
    cfg.app_data(web::Data::new(service));
    cfg.service(
        web::resource("/user/{user_id}/posts/{post_id}/comments")
        .route(get().to(handle_get_post_comments::<
            GetComments<PgRepository>
        >))
    );
}