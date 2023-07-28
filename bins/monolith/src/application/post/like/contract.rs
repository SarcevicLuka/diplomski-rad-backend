use async_trait::async_trait;
use error::Error;
use support::store::models::post_like::PostLike;

#[cfg_attr(test, mockall::automock)]
#[async_trait]
pub trait LikePostContract {
    async fn like_post(&self, user_id: &str, post_id: &str) -> Result<PostLike, Error>;
    async fn remove_like_post(&self, user_id: &str, post_id: &str) -> Result<(), Error>;
}

#[cfg_attr(test, mockall::automock)]
#[async_trait]
pub trait PgRepositoryContract {
    async fn like_post(&self, user_id: &str, post_id: &str) -> Result<PostLike, Error>;
    async fn remove_like_post(&self, user_id: &str, post_id: &str) -> Result<(), Error>;
}