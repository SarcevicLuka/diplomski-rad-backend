use std::sync::Arc;
use async_trait::async_trait;
use error::Error;
use infrastructure::db::Postgres;
use support::store::models::user::User;
use super::contract::PgRepositoryContract;

pub struct PgRepository {
    pub pg_pool: Arc<Postgres>,
}

#[async_trait]
impl PgRepositoryContract for PgRepository {
    async fn get_user_by_email(&self, email: &str) -> Result<User, Error> {
        let mut connection = match self.pg_pool.connection() {
            Ok(c) => c,
            Err(_) => return Err(Error::NotFoundWithCause("No connection".to_string())),
        };
        let exists = User::get_by_email(email, &mut connection);

        match exists {
            Ok(user) => Ok(user),
            Err(_) => return Err(Error::Request("User with given email does not exist".to_string())),
        }
    }
}