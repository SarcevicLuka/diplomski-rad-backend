use std::sync::Arc;
use async_trait::async_trait;
use diesel::{RunQueryDsl, QueryDsl, ExpressionMethods};
use error::Error;
use infrastructure::{db::Postgres, schema::{comments, posts}};
use super::contract::PgRepositoryContract;
use support::store::models::comment::Comment;


pub struct PgRepository {
    pub pg_pool: Arc<Postgres>,
}

#[async_trait]
impl PgRepositoryContract for PgRepository {
    async fn delete_comment(
        &self,
        comment_id: &str
    ) -> Result<String, Error> {
        let mut conn = self.pg_pool.connection()?;

        let post_id = comments::table
            .filter(comments::id.eq(comment_id))
            .select(comments::post_id)
            .get_result::<String>(&mut conn)?;

        let num_of_deleted_rows = Comment::delete(comment_id, conn)?;
        if num_of_deleted_rows == 0 {
            return Err(Error::Request("Comment id is invalid".to_string()));
        }

        Ok(post_id)
    } 

    async fn decrement_posts_comment(
        &self,
        post_id: &str
    ) -> Result<usize, Error> {
        let mut conn = self.pg_pool.connection()?;

        diesel::update(posts::table)
            .filter(posts::id.eq(post_id))
            .set(posts::num_of_comments.eq(posts::num_of_comments - 1))
            .execute(&mut conn)
            .map_err(Error::from)
    }
}