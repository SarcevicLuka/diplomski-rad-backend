use error::Error;
use async_trait::async_trait;
use support::store::models::user::{
    DisplayUser, 
    CreateNewUserData, 
    User
};
use super::data::RegistrationUserData;

#[cfg_attr(test, mockall::automock)]
#[async_trait]
pub trait RegistrationContract {
    async fn exists(&self, email: &str) -> bool;
    async fn register(
        &self,
        data: RegistrationUserData
    ) -> Result<(DisplayUser, String), Error>;
}

#[cfg_attr(test, mockall::automock)]
#[async_trait]
pub trait PgRepositoryContract {
    async fn exists(&self, email: &str) -> bool;
}

#[cfg_attr(test, mockall::automock)]
#[async_trait]
pub trait PgServiceContract {
    async fn create_user(&self, data: CreateNewUserData) -> Result<User, Error>;
}