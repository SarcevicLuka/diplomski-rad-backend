use std::sync::Arc;
use actix_web::{web, web::delete};
use infrastructure::db::Postgres;
use super::{
    domain::DeleteComment, 
    infrastructure::PgRepository, http::handle_delete_comment
};

#[allow(dead_code)]
pub fn routes(
    postgres: Arc<Postgres>, 
    cfg: &mut web::ServiceConfig
) {
    let service = DeleteComment {
        repository: PgRepository {
            pg_pool: postgres,
        },
    };
    
    cfg.app_data(web::Data::new(service));
    cfg.service(
        web::resource("/comments/{comment_id}/delete")
        .route(delete().to(handle_delete_comment::<
            DeleteComment<PgRepository>
        >))
        .wrap(crate::middleware::AuthLogin)
    );
}