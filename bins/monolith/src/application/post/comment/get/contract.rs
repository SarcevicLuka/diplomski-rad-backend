use async_trait::async_trait;
use error::Error;
use support::store::models::comment::Comment;
use length_aware_paginator::Response;
use super::data::{UserCommentsAttributes, PaginatedPostsCommentsResponse};

#[cfg_attr(test, mockall::automock)]
#[async_trait]
pub trait GetCommentsContract {
    async fn get_post_comments_paginated(&self, post_id: &str, attributes: UserCommentsAttributes) -> Result<PaginatedPostsCommentsResponse, Error>;
}

#[cfg_attr(test, mockall::automock)]
#[async_trait]
pub trait PgRepositoryContract {
    async fn get_post_comments_paginated(&self, post_id: &str, attributes: UserCommentsAttributes) -> Result<Response<Comment>, Error>;
}