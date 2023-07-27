use async_trait::async_trait;
use error::Error;
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

        Ok(post_like)
    }
}