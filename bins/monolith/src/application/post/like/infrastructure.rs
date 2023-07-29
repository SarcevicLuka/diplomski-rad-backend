use std::sync::Arc;
use async_trait::async_trait;
use error::Error;
use infrastructure::db::Postgres;
use super::contract::PgRepositoryContract;
use support::store::models::post_like::PostLike;
use support::store::models::post_like::CreateNewPostLikeData;

pub struct PgRepository {
    pub pg_pool: Arc<Postgres>,
}

#[async_trait]
impl PgRepositoryContract for PgRepository {
    async fn like_post(
        &self,
        user_id: &str,
        post_id: &str
    ) -> Result<PostLike, Error> {
        let conn = self.pg_pool.connection()?;

        let new_post_like_data = CreateNewPostLikeData {
            user_id: user_id.to_string(),
            post_id: post_id.to_string()
        };

        let like = PostLike::create(new_post_like_data, conn)?;

        Ok(
            like
        )
    }

    async fn remove_like_post(
        &self,
        user_id: &str,
        post_id: &str
    ) -> Result<(), Error> {
        let conn = self.pg_pool.connection()?;

        let new_post_like_data = CreateNewPostLikeData {
            user_id: user_id.to_string(),
            post_id: post_id.to_string()
        };

        let num_of_deleted_rows = PostLike::delete(new_post_like_data, conn)?;
        if num_of_deleted_rows == 0 {
            return Err(Error::Request("User or post id is invalid".to_string()));
        }

        Ok(())
    }
}