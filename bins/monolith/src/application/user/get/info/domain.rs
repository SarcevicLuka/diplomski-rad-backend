use async_trait::async_trait;
use error::Error;
use support::store::models::user::DisplayUser;
use super::contract::{PgRepositoryContract, GetUserContract};

pub struct GetUser<
    A: PgRepositoryContract,
> {
    pub repository: A,
}

#[async_trait]
impl<A> GetUserContract for GetUser<A>
where
    A: PgRepositoryContract + Send + Sync,
{
    async fn get_user_by_id(
        &self,
        user_id: &str
    ) -> Result<DisplayUser, Error> {
        let user = self
            .repository
            .get_user_by_id(user_id)
            .await?;

        Ok(
            user
        )
    }
}