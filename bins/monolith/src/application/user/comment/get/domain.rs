use async_trait::async_trait;
use error::Error;
use super::{contract::{PgRepositoryContract, GetCommentsContract}, data::{UserCommentsAttributes, PaginatedPostsCommentsResponse}};

pub struct GetComments<
    A: PgRepositoryContract,
> {
    pub repository: A,
}

#[async_trait]
impl<A> GetCommentsContract for GetComments<A>
where
    A: PgRepositoryContract + Send + Sync,
{
    async fn get_post_comments_paginated(
        &self,
        post_id: &str,
        attibutes: UserCommentsAttributes
    ) -> Result<PaginatedPostsCommentsResponse, Error> {
        self
            .repository
            .get_post_comments_paginated(post_id, attibutes)
            .await
            .map(PaginatedPostsCommentsResponse::from)
    }
}