use async_trait::async_trait;
use error::Error;
use super::{
    contract::{PgRepositoryContract, GetUserContract},
    data::UserInfoResponse
};

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
        auth_user_id: Option<String>,
        user_id: &str
    ) -> Result<UserInfoResponse, Error> {
        let am_following =  match auth_user_id {
            Some(id) => {
                self
                    .repository
                    .am_following(&id, user_id)
                    .await?
            },
            None => false
        };
        debug!("Before user fetch");
        let user = self
            .repository
            .get_user_by_id(user_id)
            .await?;

        Ok(
            UserInfoResponse {
                user_data: user,
                am_following: am_following
            }
        )
    }
}