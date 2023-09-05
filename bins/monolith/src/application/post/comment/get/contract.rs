use async_trait::async_trait;
use error::Error;
use length_aware_paginator::Response;
use super::data::{UserCommentsAttributes, PaginatedPostsCommentsResponse, CommentData};

#[cfg_attr(test, mockall::automock)]
#[async_trait]
pub trait GetCommentsContract {
    async fn get_post_comments_paginated(&self, user_id: Option<String>, post_id: &str, attributes: UserCommentsAttributes) -> Result<PaginatedPostsCommentsResponse, Error>;
}

#[cfg_attr(test, mockall::automock)]
#[async_trait]
pub trait PgRepositoryContract {
    async fn get_post_comments_paginated(&self, user_id: Option<String>, post_id: &str, attributes: UserCommentsAttributes) -> Result<Response<CommentData>, Error>;
}