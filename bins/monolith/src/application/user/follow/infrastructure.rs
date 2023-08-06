use std::sync::Arc;
use async_trait::async_trait;
use error::Error;
use infrastructure::db::Postgres;
use support::store::models::{user_follow::{UserFollow, CreateNewUserFollowData}, user::User};
use super::contract::PgRepositoryContract;

pub struct PgRepository {
    pub pg_pool: Arc<Postgres>,
}

#[async_trait]
impl PgRepositoryContract for PgRepository {
    async fn follow_user(
        &self,
        request_user_id: &str,
        response_user_id: &str
    ) -> Result<UserFollow, Error> {
        let conn = self.pg_pool.connection()?;

        let responding_user = User::get_by_id(response_user_id, &mut self.pg_pool.connection()?);

        match responding_user {
            Ok(_user) => {},
            Err(_error) => return Err(Error::Request("No user found with given id".to_string())),
        }

        let new_user_follow_data = CreateNewUserFollowData {
            user_requesting: request_user_id.to_string(),
            user_responding: response_user_id.to_string()
        };


        let user_follow = UserFollow::create(new_user_follow_data, conn)?;

        Ok(
            user_follow
        )
    }

    async fn unfollow_user(
        &self,
        user_id: &str,
        unfollowed_user_id: &str
    ) -> Result<usize, Error> {
        let conn = self.pg_pool.connection()?;

        UserFollow::delete(user_id, unfollowed_user_id, conn)
    }
}