use async_trait::async_trait;
use error::Error;
use super::contract::{DeletePostContract, PgRepositoryContract};

pub struct DeletePost<
    A: PgRepositoryContract,
> {
    pub repository: A,
}

#[async_trait]
impl<A> DeletePostContract for DeletePost<A>
where
    A: PgRepositoryContract + Send + Sync,
{
    async fn delete_post(
        &self,
        post_id: &str,
    ) -> Result<(), Error> {
        self
            .repository
            .delete_post(post_id)
            .await?;

        Ok(())
    }
}