use async_trait::async_trait;
use error::Error;
use support::store::models::post::Post;

#[cfg_attr(test, mockall::automock)]
#[async_trait]
pub trait GetPostsContract {
    async fn get_post(&self, post_id: &str) -> Result<Post, Error>;
    //async fn get_all_posts(&self, user_id: &str) -> Result<Post, Error>;
}

#[cfg_attr(test, mockall::automock)]
#[async_trait]
pub trait PgRepositoryContract {
    async fn get_post(&self, post_id: &str) -> Result<Post, Error>;
    //async fn get_all_posts(&self, user_id: &str) -> Result<Post, Error>;
}