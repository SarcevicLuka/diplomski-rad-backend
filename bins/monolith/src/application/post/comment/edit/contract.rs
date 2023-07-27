use async_trait::async_trait;
use error::Error;
use support::store::models::comment::{Comment, DisplayComment};

use super::data::UserEditCommentData;

#[cfg_attr(test, mockall::automock)]
#[async_trait]
pub trait EditCommentContract {
    async fn edit_comment(&self, comment_id: &str, edit_comment_data: UserEditCommentData) -> Result<DisplayComment, Error>;
}

#[cfg_attr(test, mockall::automock)]
#[async_trait]
pub trait PgRepositoryContract {
    async fn edit_comment(&self, comment_id: &str, edited_text: &str, score: &i32) -> Result<Comment, Error>;
}