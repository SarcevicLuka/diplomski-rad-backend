use async_trait::async_trait;
use error::Error;
use support::store::models::post::Post;
use length_aware_paginator::Response;
use super::data::{UserPostsAttributes, PaginatedUsersPostsResponse};

#[cfg_attr(test, mockall::automock)]
#[async_trait]
pub trait GetPostsContract {
    async fn get_post(&self, post_id: &str) -> Result<Post, Error>;
    async fn get_users_posts_paginated(&self, user_id: &str, attibutes: UserPostsAttributes) -> Result<PaginatedUsersPostsResponse, Error>;
}

#[cfg_attr(test, mockall::automock)]
#[async_trait]
pub trait PgRepositoryContract {
    async fn get_post(&self, post_id: &str) -> Result<Post, Error>;
    async fn get_users_posts_paginated(&self, user_id: &str, attibutes: UserPostsAttributes) -> Result<Response<Post>, Error>;
}