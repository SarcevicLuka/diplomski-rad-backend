use async_trait::async_trait;
use error::Error;
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
        self
            .repository
            .delete_comment(comment_id)
            .await?;

        Ok(())
    }
}