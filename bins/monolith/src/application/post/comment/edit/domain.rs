use async_trait::async_trait;
use error::Error;
use super::{contract::{EditCommentContract, PgRepositoryContract}, data::UserEditCommentData};
use support::store::models::comment::DisplayComment;

pub struct EditComment<
    A: PgRepositoryContract,
> {
    pub repository: A,
}

#[async_trait]
impl<A> EditCommentContract for EditComment<A>
where
    A: PgRepositoryContract + Send + Sync,
{
    async fn edit_comment(
        &self,
        comment_id: &str,
        edit_comment_data: UserEditCommentData,
    ) -> Result<DisplayComment, Error> {
        let text = match &edit_comment_data.text {
            Some(text) => text.to_string(),
            None => return Err(Error::Request("Invalid text".to_string())),
        };

        let score = match &edit_comment_data.score {
            Some(score) => score.to_owned(),
            None => return Err(Error::Request("Invalid score".to_string())),
        };

        let comment = self
            .repository
            .edit_comment(comment_id, &text, &score)
            .await?;

        Ok(
            DisplayComment::from(comment)
        )
    }
}