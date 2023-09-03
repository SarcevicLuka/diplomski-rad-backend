use async_trait::async_trait;
use error::Error;
use support::store::models::comment_like::CommentLike;

#[cfg_attr(test, mockall::automock)]
#[async_trait]
pub trait LikeCommentContract {
    async fn like_comment(&self, user_id: &str, comment_id: &str) -> Result<CommentLike, Error>;
    async fn remove_like_comment(&self, user_id: &str, comment_id: &str) -> Result<(), Error>;
}

#[cfg_attr(test, mockall::automock)]
#[async_trait]
pub trait PgRepositoryContract {
    async fn like_comment(&self, user_id: &str, comment_id: &str) -> Result<CommentLike, Error>;
    async fn remove_like_comment(&self, user_id: &str, comment_id: &str) -> Result<usize, Error>;
    async fn increment_comment_like(&self, comment_id: &str) -> Result<usize, Error>;
    async fn decrement_comment_like(&self, comment_id: &str) -> Result<usize, Error>;
}