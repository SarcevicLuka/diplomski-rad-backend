use async_trait::async_trait;
use error::Error;
use super::{contract::{PgRepositoryContract, GetPostsContract}, data::{UserPostsAttributes, PaginatedUsersPostsResponse}};

pub struct GetPost<
    A: PgRepositoryContract,
> {
    pub repository: A,
}

#[async_trait]
impl<A> GetPostsContract for GetPost<A>
where
    A: PgRepositoryContract + Send + Sync,
{
    async fn get_users_posts_paginated(
        &self,
        user_id: &str,
        attibutes: UserPostsAttributes
    ) -> Result<PaginatedUsersPostsResponse, Error> {
        self
            .repository
            .get_users_posts_paginated(user_id, attibutes)
            .await
            .map(PaginatedUsersPostsResponse::from)
    }
}