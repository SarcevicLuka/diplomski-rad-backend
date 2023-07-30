use std::sync::Arc;
use async_trait::async_trait;
use diesel::{ExpressionMethods, QueryDsl};
use error::Error;
use length_aware_paginator::{Response, Paginate};
use infrastructure::{
    db::Postgres,
    schema::users
};
use support::store::models::{
    user_follow::UserFollow, 
    user::User
};
use super::{
    contract::PgRepositoryContract, 
    data::UserFollowsAttributes
};

pub struct PgRepository {
    pub pg_pool: Arc<Postgres>,
}

#[async_trait]
impl PgRepositoryContract for PgRepository {
    async fn get_followed_users(
        &self,
        user_id: &str,
        attributes: UserFollowsAttributes
    ) -> Result<Response<User>, Error> {
        let mut conn = self.pg_pool.connection()?;

        let user_ids_followed = UserFollow::get_followed_user_ids(user_id, self.pg_pool.connection()?)?;

        let query = users::table
            .filter(users::id.eq_any(user_ids_followed))
            .order(users::first_name.desc())
            .into_boxed();

        query
            .page(attributes.page)
            .per_page(attributes.per_page)
            .paginate(&mut conn)
            .map_err(Error::from)
    }

    async fn get_following_users(
        &self,
        user_id: &str,
        attributes: UserFollowsAttributes
    ) -> Result<Response<User>, Error> {
        let mut conn = self.pg_pool.connection()?;

        let user_ids_following = UserFollow::get_following_user_ids(user_id, self.pg_pool.connection()?)?;

        let query = users::table
            .filter(users::id.eq_any(user_ids_following))
            .order(users::first_name.desc())
            .into_boxed();

        query
            .page(attributes.page)
            .per_page(attributes.per_page)
            .paginate(&mut conn)
            .map_err(Error::from)
    }
}