use async_trait::async_trait;
use error::Error;
use support::store::models::comment::{CreateNewCommentData, Comment, DisplayComment};

use super::data::UserCommentData;

#[cfg_attr(test, mockall::automock)]
#[async_trait]
pub trait CreateCommentContract {
    async fn create_comment(&self, user_id: &str, post_id: &str, comment_data: UserCommentData) -> Result<DisplayComment, Error>;
}

#[cfg_attr(test, mockall::automock)]
#[async_trait]
pub trait PgRepositoryContract {
    async fn create_comment(&self, comment_data: CreateNewCommentData) -> Result<Comment, Error>;
}