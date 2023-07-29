use std::sync::Arc;
use actix_web::{web, web::patch};
use infrastructure::db::Postgres;
use super::{
    domain::EditComment, 
    infrastructure::PgRepository, http::handle_edit_comment
};

#[allow(dead_code)]
pub fn routes(
    postgres: Arc<Postgres>, 
    cfg: &mut web::ServiceConfig
) {
    let service = EditComment {
        repository: PgRepository {
            pg_pool: postgres,
        },
    };
    
    cfg.app_data(web::Data::new(service));
    cfg.service(
        web::resource("/comments/{comment_id}/edit")
        .route(patch().to(handle_edit_comment::<
            EditComment<PgRepository>
        >))
        .wrap(crate::middleware::AuthLogin)
    );
}