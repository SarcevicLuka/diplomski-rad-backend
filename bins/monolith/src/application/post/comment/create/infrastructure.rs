use std::sync::Arc;
use async_trait::async_trait;
use diesel::{RunQueryDsl, ExpressionMethods};
use error::Error;
use infrastructure::{db::Postgres, schema::posts};
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

    async fn increment_posts_comment(
        &self,
        post_id: &str
    ) -> Result<usize, Error> {
        let mut conn = self.pg_pool.connection()?;

        diesel::update(posts::table)
            .filter(posts::id.eq(post_id))
            .set(posts::num_of_comments.eq(posts::num_of_comments + 1))
            .execute(&mut conn)
            .map_err(Error::from)
    }
}