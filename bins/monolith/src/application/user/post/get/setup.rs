use std::sync::Arc;
use actix_web::{web, web::get};
use infrastructure::db::Postgres;
use super::{
    domain::GetPost, 
    infrastructure::PgRepository, http::handle_get_post
};

#[allow(dead_code)]
pub fn routes(
    postgres: Arc<Postgres>, 
    cfg: &mut web::ServiceConfig
) {
    let service = GetPost {
        repository: PgRepository {
            pg_pool: postgres,
        },
    };
    
    cfg.app_data(web::Data::new(service));
    cfg.service(
        web::resource("/{user_id}/posts/{post_id}")
        .route(get().to(handle_get_post::<
            GetPost<PgRepository>
        >))
    );
    //cfg.service(
    //    web::resource("/{user_id}/posts")
    //    .route(get().to(handle_get_all_posts::<
    //        GetPost<PgRepository>
    //    >))
    //    .wrap(crate::middleware::AuthLogin)
    //);
}