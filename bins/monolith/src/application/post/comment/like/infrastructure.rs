use std::sync::Arc;
use async_trait::async_trait;
use error::Error;
use infrastructure::db::Postgres;
use support::store::models::comment_like::{CommentLike, CreateNewCommentLikeData};
use super::contract::PgRepositoryContract;

pub struct PgRepository {
    pub pg_pool: Arc<Postgres>,
}

#[async_trait]
impl PgRepositoryContract for PgRepository {
    async fn like_comment(
        &self,
        user_id: &str,
        comment_id: &str
    ) -> Result<CommentLike, Error> {
        let conn = self.pg_pool.connection()?;

        let new_post_like_data = CreateNewCommentLikeData {
            user_id: user_id.to_string(),
            comment_id: comment_id.to_string()
        };

        let like = CommentLike::create(new_post_like_data, conn)?;

        Ok(
            like
        )
    }

    async fn remove_like_comment(
        &self,
        user_id: &str,
        comment_id: &str
    ) -> Result<usize, Error> {
        let conn = self.pg_pool.connection()?;

        let new_comment_like_data = CreateNewCommentLikeData {
            user_id: user_id.to_string(),
            comment_id: comment_id.to_string()
        };

        CommentLike::delete(new_comment_like_data, conn)
    }
}