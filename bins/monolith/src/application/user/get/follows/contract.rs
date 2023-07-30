use async_trait::async_trait;
use error::Error;
use length_aware_paginator::Response;
use support::store::models::user::User;
use super::data::{PaginatedFollowsResponse, UserFollowsAttributes};

#[cfg_attr(test, mockall::automock)]
#[async_trait]
pub trait GetFollowsContract {
    async fn get_followed_users(&self, user_id: &str, attributes: UserFollowsAttributes) -> Result<PaginatedFollowsResponse, Error>;
    async fn get_following_users(&self, user_id: &str, attributes: UserFollowsAttributes) -> Result<PaginatedFollowsResponse, Error>;
}

#[cfg_attr(test, mockall::automock)]
#[async_trait]
pub trait PgRepositoryContract {
    async fn get_followed_users(&self, user_id: &str, attributes: UserFollowsAttributes) -> Result<Response<User>, Error>;
    async fn get_following_users(&self, user_id: &str, attributes: UserFollowsAttributes) -> Result<Response<User>, Error>;
}