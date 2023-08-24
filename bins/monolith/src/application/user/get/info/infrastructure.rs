use std::sync::Arc;
use async_trait::async_trait;
use diesel::{QueryDsl, RunQueryDsl, ExpressionMethods, BoolExpressionMethods};
use error::Error;
use infrastructure::{db::Postgres, schema::friendships};
use support::store::models::user::{User, DisplayUser};
use super::contract::PgRepositoryContract;


pub struct PgRepository {
    pub pg_pool: Arc<Postgres>,
}

#[async_trait]
impl PgRepositoryContract for PgRepository {
    /// Fetches user by id
    async fn get_user_by_id(
        &self,
        user_id: &str
    ) -> Result<DisplayUser, Error> {
        let mut conn = self.pg_pool.connection()?;

        let user = User::get_by_id(user_id, &mut conn)?;

        Ok(
            DisplayUser::from(user)
        )
    } 

    async fn am_following(
        &self,
        auth_user_id: &str,
        user_id: &str
    ) -> Result<bool, Error> {
        let mut conn = self.pg_pool.connection()?;

        let friendship_id = friendships::table
            .filter(
                friendships::user_requesting.eq(auth_user_id)
                .and(friendships::user_responding.eq(user_id))
            )
            .select(friendships::id)
            .get_result::<String>(&mut conn)
            .map_err(Error::from);

        if friendship_id?.len() > 0 {
            return Ok(true);
        }
        else {
            return Ok(false)
        };
    }
}