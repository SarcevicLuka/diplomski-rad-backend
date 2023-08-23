use async_trait::async_trait;
use error::Error;
use diesel::result::Error as DieselError;
use super::contract::{DeleteCommentContract, PgRepositoryContract};

pub struct DeleteComment<
    A: PgRepositoryContract,
> {
    pub repository: A,
}

#[async_trait]
impl<A> DeleteCommentContract for DeleteComment<A>
where
    A: PgRepositoryContract + Send + Sync,
{
    async fn delete_comment(
        &self,
        comment_id: &str,
    ) -> Result<(), Error> {
        let post_id = self
            .repository
            .delete_comment(comment_id)
            .await?;

        let num_of_rows_decremented = self
            .repository
            .decrement_posts_comment(&post_id)
            .await?;

        if num_of_rows_decremented == 0 {
            return Err(Error::Diesel(DieselError::NotFound));
        }

        Ok(())
    }
}