use std::sync::Arc;
use actix_web::{web, web::get};
use infrastructure::db::Postgres;

use super::{
    domain::GetLikes, 
    infrastructure::PgRepository, http::handle_get_users_likes
};

#[allow(dead_code)]
pub fn routes(
    postgres: Arc<Postgres>, 
    cfg: &mut web::ServiceConfig
) {
    let service = GetLikes {
        repository: PgRepository {
            pg_pool: postgres,
        },
    };
    
    cfg.app_data(web::Data::new(service));
    cfg.service(
        web::resource("/user/{user_id}/likes")
        .route(get().to(handle_get_users_likes::<
            GetLikes<PgRepository>
        >))
    );
}