use async_trait::async_trait;
use error::Error;
use super::{contract::{CreatePostContract, PgRepositoryContract}, data::UserPostData};
use support::store::models::post::{Post, CreateNewPostData};

pub struct CreatePost<
    A: PgRepositoryContract,
> {
    pub repository: A,
}

#[async_trait]
impl<A> CreatePostContract for CreatePost<A>
where
    A: PgRepositoryContract + Send + Sync,
{
    async fn create_post(
        &self,
        user_id: &str,
        post_data: UserPostData,
    ) -> Result<Post, Error> {
        let watch_id = self
            .repository
            .create_watch(post_data.watch_data.insertable())
            .await?;

        let post_data = CreateNewPostData {
            user_id: user_id.to_string(),
            watch_id,
            review: post_data.review.unwrap(),
            score: post_data.score.unwrap()
        };

        let post = self
            .repository
            .create_post(post_data)
            .await?;

        Ok(
            post
        )
    }
}