use std::sync::Arc;
use async_trait::async_trait;
use error::Error;
use infrastructure::db::Postgres;
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
}