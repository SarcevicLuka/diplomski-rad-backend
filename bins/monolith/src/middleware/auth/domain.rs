use super::contract::{
    RequestContract, 
    SessionContract, 
    PgRepositoryContract
};
use support::store::models::user::AuthenticatedUser;
use error::Error;

pub struct Session<A, B> {
    pub request: A,
    pub repository: B,
}

impl<A, B> SessionContract for Session<A, B>
where
    A: RequestContract,
    B: PgRepositoryContract,
{
    /// Extract user email from jwt and find that user in database
    fn extract_valid_user(&mut self) -> Result<AuthenticatedUser, Error> {
        let user_email = self.request.try_extract_email_from_jwt()?;

        let user = match self.repository.get_user_by_email(&user_email) {
            Ok(user) => user,
            Err(_) => return Err(
                Error::Forbidden("user from jwt not found in database".to_string())
            ),
        };
        let authenticated_user = AuthenticatedUser::from(user);

        Ok(authenticated_user)
    }
}