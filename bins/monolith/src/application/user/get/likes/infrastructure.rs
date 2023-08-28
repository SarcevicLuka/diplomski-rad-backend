use std::sync::Arc;
use async_trait::async_trait;
use error::Error;
use infrastructure::db::Postgres;
use support::store::models::post_like::PostLike;
use super::{contract::PgRepositoryContract, data::UserLikes};

pub struct PgRepository {
    pub pg_pool: Arc<Postgres>,
}

#[async_trait]
impl PgRepositoryContract for PgRepository {
    /// Fetches user by id
    async fn get_user_likes(
        &self,
        user_id: &str
    ) -> Result<UserLikes, Error> {
        let conn = self.pg_pool.connection()?;
        
        let user_likes = PostLike::get_post_likes(user_id, conn)?;

        Ok(
            UserLikes { 
                user_likes
            }
        )
    }
}