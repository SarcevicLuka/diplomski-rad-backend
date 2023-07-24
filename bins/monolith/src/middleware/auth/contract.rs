//use async_trait::async_trait;
use error::Error;
use support::store::models::user::{User, DisplayUser};

#[cfg_attr(test, mockall::automock)]
pub trait SessionContract {
    fn extract_valid_user(&mut self) -> Result<DisplayUser, Error>;
}

#[cfg_attr(test, mockall::automock)]
pub trait RequestContract {
    fn try_extract_email_from_jwt(&self) -> Result<String, Error>;
}

#[cfg_attr(test, mockall::automock)]
pub trait PgRepositoryContract {
    fn get_user_by_email(&mut self, email: &str) -> Result<User, Error>;
}