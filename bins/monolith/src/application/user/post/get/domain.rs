use async_trait::async_trait;
use error::Error;
use super::contract::{PgRepositoryContract, GetPostsContract};
use support::store::models::post::Post;

pub struct GetPost<
    A: PgRepositoryContract,
> {
    pub repository: A,
}

#[async_trait]
impl<A> GetPostsContract for GetPost<A>
where
    A: PgRepositoryContract + Send + Sync,
{
    async fn get_post(
        &self,
        post_id: &str
    ) -> Result<Post, Error> {
        let post = self
            .repository
            .get_post(post_id)
            .await?;

        Ok(
            post
        )
    }
}