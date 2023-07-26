use async_trait::async_trait;
use error::Error;
use super::{contract::{CreateCommentContract, PgRepositoryContract}, data::UserCommentData};
use support::store::models::comment::{CreateNewCommentData, DisplayComment};

pub struct CreateComment<
    A: PgRepositoryContract,
> {
    pub repository: A,
}

#[async_trait]
impl<A> CreateCommentContract for CreateComment<A>
where
    A: PgRepositoryContract + Send + Sync,
{
    async fn create_comment(
        &self,
        user_id: &str,
        post_id: &str,
        comment_data: UserCommentData,
    ) -> Result<DisplayComment, Error> {
        let text = match &comment_data.text {
            Some(text) => text.to_string(),
            None => return Err(Error::Request("Invalid text".to_string())),
        };

        let score = match &comment_data.score {
            Some(text) => text.to_owned(),
            None => return Err(Error::Request("Invalid score".to_string())),
        };

        let create_comment_data = CreateNewCommentData {
            user_id: user_id.to_string(),
            post_id: post_id.to_string(),
            text,
            score
        };

        let comment = self
            .repository
            .create_comment(create_comment_data)
            .await?;

        Ok(
            DisplayComment::from(comment)
        )
    }
}