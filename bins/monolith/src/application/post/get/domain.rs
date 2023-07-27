use async_trait::async_trait;
use error::Error;
use super::{contract::{PgRepositoryContract, GetPostsContract}, data::{UserPostsAttributes, PaginatedUsersPostsResponse}};
use support::store::models::post::DisplayPost;

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
    async fn get_post(
        &self,
        post_id: &str
    ) -> Result<DisplayPost, Error> {
        let post = self
            .repository
            .get_post(post_id)
            .await?;

        Ok(
            post
        )
    }

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