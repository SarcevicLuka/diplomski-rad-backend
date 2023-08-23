use std::sync::Arc;
use async_trait::async_trait;
use diesel::{ExpressionMethods, RunQueryDsl};
use error::Error;
use infrastructure::{
    db::Postgres, 
    schema::posts
};
use support::store::models::post_like::{PostLike, CreateNewPostLikeData};
use super::contract::PgRepositoryContract;

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
    ) -> Result<usize, Error> {
        let conn = self.pg_pool.connection()?;

        let new_post_like_data = CreateNewPostLikeData {
            user_id: user_id.to_string(),
            post_id: post_id.to_string()
        };

        PostLike::delete(new_post_like_data, conn)
    }

    async fn increment_posts_like(
        &self,
        post_id: &str
    ) -> Result<usize, Error> {
        let mut conn = self.pg_pool.connection()?;

        diesel::update(posts::table)
            .filter(posts::id.eq(post_id))
            .set(posts::num_of_likes.eq(posts::num_of_likes + 1))
            .execute(&mut conn)
            .map_err(Error::from)
    }

    async fn decrement_posts_like(
        &self,
        post_id: &str
    ) -> Result<usize, Error> {
        let mut conn = self.pg_pool.connection()?;

        diesel::update(posts::table)
            .filter(posts::id.eq(post_id))
            .set(posts::num_of_likes.eq(posts::num_of_likes + 1))
            .execute(&mut conn)
            .map_err(Error::from)
    }
}