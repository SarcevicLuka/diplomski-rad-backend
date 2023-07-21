use std::sync::Arc;
use async_trait::async_trait;
use diesel::{ExpressionMethods, RunQueryDsl, QueryDsl};
use error::Error;
use infrastructure::{schema::users, db::Postgres};
use support::store::models::user::{User, CreateNewUserData};
use super::contract::{PgRepositoryContract, PgServiceContract};

pub struct PgRepository {
    pub pg_pool: Arc<Postgres>,
}

#[async_trait]
impl PgRepositoryContract for PgRepository {
    async fn exists(&self, email: &str) -> bool {
        let mut connection = match self.pg_pool.connection() {
            Ok(c) => c,
            Err(_) => return false,
        };

        match users::table
            .filter(users::email.eq(email))
            .count()
            .get_result::<i64>(&mut connection)
        {
            Ok(count) => count > 0,
            Err(_e) => false,
        }
    }
}

pub struct PgService {
    pub pg_pool: Arc<Postgres>,
}

#[async_trait]
impl PgServiceContract for PgService {
    async fn create_user(&self, data: CreateNewUserData) -> Result<User, Error> {
        User::create(data, self.pg_pool.connection()?)
    }
}