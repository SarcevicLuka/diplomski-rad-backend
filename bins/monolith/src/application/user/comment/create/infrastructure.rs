use std::sync::Arc;
use async_trait::async_trait;
use error::Error;
use infrastructure::db::Postgres;
use support::store::models::comment::{CreateNewCommentData, Comment};
use super::contract::PgRepositoryContract;


pub struct PgRepository {
    pub pg_pool: Arc<Postgres>,
}

#[async_trait]
impl PgRepositoryContract for PgRepository {
    /// Fetches pokemon by id
    async fn create_comment(
        &self,
        comment_data: CreateNewCommentData
    ) -> Result<Comment, Error> {
        let conn = self.pg_pool.connection()?;

        let comment = Comment::create(comment_data, conn)?;

        Ok(
            comment
        )
    }
}