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
    async fn delete_comment(&self, comment_id: &str) -> Result<(), Error>;
}