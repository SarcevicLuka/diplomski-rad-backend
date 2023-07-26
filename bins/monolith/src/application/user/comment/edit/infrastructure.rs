use std::sync::Arc;
use async_trait::async_trait;
use error::Error;
use infrastructure::db::Postgres;
use support::store::models::comment::Comment;
use super::contract::PgRepositoryContract;


pub struct PgRepository {
    pub pg_pool: Arc<Postgres>,
}

#[async_trait]
impl PgRepositoryContract for PgRepository {
    /// Edits comment
    async fn edit_comment(
        &self,
        comment_id: &str,
        text: &str,
        score: &i32
    ) -> Result<Comment, Error> {
        let mut conn = self.pg_pool.connection()?;

        let comment = Comment::edit_by_id(comment_id, text, score, &mut conn)?;

        Ok(
            comment
        )
    }
}