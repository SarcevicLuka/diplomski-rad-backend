use error::Error;
use async_trait::async_trait;
use support::store::models::user::{DisplayUser, User};
use super::{contract::{
    PgRepositoryContract, 
    PgServiceContract, 
    RegistrationContract
}, data::RegistrationUserData};

pub struct Registration<
    A: PgRepositoryContract,
    B: PgServiceContract,
> {
    pub repository: A,
    pub service: B,
}

#[async_trait]
impl <A, B> RegistrationContract for Registration<A, B>
where
    A: PgRepositoryContract + Send + Sync,
    B: PgServiceContract + Send + Sync,
{
    async fn exists(&self, email: &str) -> bool {
        self.repository.exists(&email.to_lowercase()).await
    }

    async fn register(
        &self, 
        mut data: RegistrationUserData
    ) -> Result<(DisplayUser, String), Error> {
        let email = match &data.email {
            Some(email) => email.to_string().to_lowercase(),
            None => return Err(Error::Request("Invalid credentials".to_string())),
        };
        let password = match &data.password {
            Some(password) => password.to_string(),
            None => return Err(Error::Request("Invalid credentials".to_string())),
        };

        if self.exists(&email).await {
            return Err(Error::Request("Email is already in use".to_string()));
        }

        data.password = Some(User::hash_password(&password)?);

        let auth_user = self.service
            .create_user(data.insertable())
            .await
            .map(DisplayUser::from)?;
    
        let token = User::generate_jwt_token(&auth_user)?;

        Ok((auth_user, token))
    }
}