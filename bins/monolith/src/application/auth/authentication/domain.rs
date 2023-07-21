use async_trait::async_trait;
use error::Error;
use support::store::models::user::{AuthenticatedUser, User};
use crate::application::auth::authentication::contract::PgRepositoryContract;
use super::{
    contract::LoginContract, 
    data::LoginUserData
};

pub struct Login <
    A: PgRepositoryContract
> {
    pub repository: A,
}

#[async_trait]
impl<A> LoginContract for Login<A>
where
    A: PgRepositoryContract + Send + Sync,
{
    async fn get_user_by_email(&self, email: &str) -> Result<User, Error> {
        self.repository.get_user_by_email(&email.to_lowercase()).await
    }

    async fn login(
        &self,
        data: LoginUserData
    ) -> Result<(AuthenticatedUser, String), Error> {
        let email = match &data.email {
            Some(email) => email.to_string().to_lowercase(),
            None => return Err(Error::Request("Invalid credentials".to_string())),
        };
        let password = match &data.password {
            Some(password) => password.to_string(),
            None => return Err(Error::Request("Invalid credentials".to_string())),
        };

        let user = self.repository
            .get_user_by_email(&email)
            .await?;

        if !User::verify_password(&password, &user.password) {
            return Err(Error::Forbidden("Invalid credentials".to_string()));
        };
        
        let authenticated_user = AuthenticatedUser::from(user);
        let token = User::generate_jwt_token(&authenticated_user)?;

        Ok((authenticated_user, token))
    }
}