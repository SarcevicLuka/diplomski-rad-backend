use async_trait::async_trait;
use error::Error;
use super::{
    contract::{PgRepositoryContract, GetUserLikesContract},
    data::UserLikes
};

pub struct GetLikes<
    A: PgRepositoryContract,
> {
    pub repository: A,
}

#[async_trait]
impl<A> GetUserLikesContract for GetLikes<A>
where
    A: PgRepositoryContract + Send + Sync,
{
    async fn get_user_likes(
        &self,
        user_id: &str
    ) -> Result<UserLikes, Error> {
        self
            .repository
            .get_user_likes(user_id)
            .await
    }
}