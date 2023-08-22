use async_trait::async_trait;
use error::Error;

#[cfg_attr(test, mockall::automock)]
#[async_trait]
pub trait DeleteCommentContract {
    async fn delete_comment(&self, comment_id: &str) -> Result<(), Error>;
}

#[cfg_attr(test, mockall::automock)]
#[async_trait]
pub trait PgRepositoryContract {
    async fn delete_comment(&self, comment_id: &str) -> Result<String, Error>;
    async fn decrement_posts_comment(&self, post_id_id: &str) -> Result<usize, Error>;
}