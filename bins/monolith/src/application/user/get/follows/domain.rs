use async_trait::async_trait;
use error::Error;
use super::{
    contract::{GetFollowsContract, PgRepositoryContract}, 
    data::{PaginatedFollowsResponse, UserFollowsAttributes}
};

pub struct GetFollows<
    A: PgRepositoryContract,
> {
    pub repository: A,
}

#[async_trait]
impl<A> GetFollowsContract for GetFollows<A>
where
    A: PgRepositoryContract + Send + Sync,
{
    async fn get_followed_users(
        &self,
        user_id: &str,
        attribues: UserFollowsAttributes
    ) -> Result<PaginatedFollowsResponse, Error> {
        self
            .repository
            .get_followed_users(user_id, attribues)
            .await
            .map(PaginatedFollowsResponse::from)
    }

    async fn get_following_users(
        &self,
        user_id: &str,
        attribues: UserFollowsAttributes
    ) -> Result<PaginatedFollowsResponse, Error> {
        self
            .repository
            .get_following_users(user_id, attribues)
            .await
            .map(PaginatedFollowsResponse::from)
    }
}