use std::sync::Arc;
use actix_web::{web, web::delete};
use infrastructure::db::Postgres;
use super::{
    domain::DeletePost, 
    infrastructure::PgRepository, http::handle_delete_post
};

#[allow(dead_code)]
pub fn routes(
    postgres: Arc<Postgres>, 
    cfg: &mut web::ServiceConfig
) {
    let service = DeletePost {
        repository: PgRepository {
            pg_pool: postgres,
        },
    };
    
    cfg.app_data(web::Data::new(service));
    cfg.service(
        web::resource("/posts/{post_id}/delete")
        .route(delete().to(handle_delete_post::<
            DeletePost<PgRepository>
        >))
        .wrap(crate::middleware::AuthLogin)
    );
}