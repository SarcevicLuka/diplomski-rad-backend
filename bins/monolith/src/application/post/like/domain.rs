use async_trait::async_trait;
use error::Error;
use diesel::result::Error as DieselError;
use super::contract::{LikePostContract, PgRepositoryContract};
use support::store::models::post_like::PostLike;

pub struct LikePost<
    A: PgRepositoryContract,
> {
    pub repository: A,
}

#[async_trait]
impl<A> LikePostContract for LikePost<A>
where
    A: PgRepositoryContract + Send + Sync,
{
    async fn like_post(
        &self,
        user_id: &str,
        post_id: &str,
    ) -> Result<PostLike, Error> {
        let post_like = self
            .repository
            .like_post(user_id, post_id)
            .await?;

        let num_of_rows_incremented = self
            .repository
            .increment_posts_like(post_id)
            .await?;

        if num_of_rows_incremented != 0 {
            return Err(Error::Diesel(DieselError::NotFound));
        }

        Ok(post_like)
    }

    async fn remove_like_post(
        &self,
        user_id: &str,
        post_id: &str,
    ) -> Result<(), Error> {
        let num_of_deleted_rows = self
            .repository
            .remove_like_post(user_id, post_id)
            .await?;

        if num_of_deleted_rows == 0 {
            return Err(Error::Request("User or post id is invalid".to_string()));
        }

        let num_of_rows_incremented = self
            .repository
            .decrement_posts_like(post_id)
            .await?;

        if num_of_rows_incremented != 0 {
            return Err(Error::Diesel(DieselError::NotFound));
        }

        Ok(())
    }
}