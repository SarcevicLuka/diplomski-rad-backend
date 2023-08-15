use super::contract::{RequestContract, PgRepositoryContract};
use actix_web::dev::ServiceRequest;
use infrastructure::db::DbConnection;
use error::Error;
use support::{
    helpers::jwt, 
    store::models::user::User
};

pub struct Request<'a> {
    pub req: &'a ServiceRequest,
}

impl<'a> RequestContract for Request<'a> {
    /// Function to extract user email from jwt token
    fn try_extract_email_from_jwt(&self) -> Result<String, Error> {
        let header = match &self.req.headers().get("Authorization") {
            Some(head) => match head.to_str().ok() {
                Some(val) => val.to_string(),
                None => return Err(
                    Error::Request("Authorization header empty".to_string())
                ),
            },
            None => return Err(
                Error::Forbidden("No authorization header".to_string())
            ),
        };
        let mut split = header.split_whitespace();
        let auth_type = split.next();
        if Some("Bearer") == auth_type {
            if let Some(token) = split.next() {
                let secret = config::get_default("JWT_SECRET", "not_so_strong_secret");
                let claims = jwt::verify(token.to_string(), &secret)?;
                Ok(claims.email)
            } else {
                Err(Error::Unauthorized("No jwt token".to_string()))
            }
        } else {
            Err(Error::Forbidden("No bearer scheme with jwt token".to_string()))
        }
    }
}

pub struct PgRepository {
    pub pg_connection: DbConnection,
}

impl PgRepositoryContract for PgRepository {
    fn get_user_by_email(&mut self, email: &str) -> Result<User, Error> {
        let exists = User::get_by_email(email, &mut self.pg_connection);

        match exists {
            Ok(user) => Ok(user),
            Err(_) => Err(Error::NotFound),
        }
    }
}

#[cfg(test)]
mod test {
    use super::Request;
    use actix_web::http;
    use error::Error;
    use support::helpers::jwt::testable as jwt_testable;
    use crate::middleware::auth::contract::RequestContract;

    #[test]
    fn test_with_valid_jwt_token_email_given() {
        let jwt = jwt_testable();

        let req = actix_web::test::TestRequest::default()
            .insert_header((http::header::AUTHORIZATION, format!("Bearer {jwt}")))
            .to_srv_request();

        let request = Request {
            req: &req,
        };

        let response = request.try_extract_email_from_jwt().unwrap();
        
        assert_eq!(response, "test@test.com")
    }

    #[test]
    fn test_with_invalid_jwt_token() {
        let mut jwt = jwt_testable();
        jwt.push_str("invalidSignature");


        let req = actix_web::test::TestRequest::default()
            .insert_header((http::header::AUTHORIZATION, format!("Bearer {jwt}")))
            .to_srv_request();

        let request = Request {
            req: &req,
        };

        let response = request.try_extract_email_from_jwt();
        match response {
            Ok(_email) => panic!("Should have been an error"),
            Err(e) => match e {
                Error::Jwt => {},
                _ => panic!("Wrong error"),
            }
        };
    }

    #[test]
    fn test_with_no_authorization_header() {
        let req = actix_web::test::TestRequest::default()
            .to_srv_request();

        let request = Request {
            req: &req,
        };

        let response = request.try_extract_email_from_jwt();
        match response {
            Ok(_email) => panic!("Should have been an error"),
            Err(e) => match e {
                Error::Forbidden(cause) => {
                    assert_eq!(cause, "No authorization header".to_string())
                },
                _ => panic!("Wrong error"),
            }
        };
    }

    #[test]
    fn test_with_no_bearer_schema_in_header() {
        let jwt = jwt_testable();

        let req = actix_web::test::TestRequest::default()
            .insert_header((http::header::AUTHORIZATION, format!("{jwt}")))
            .to_srv_request();

        let request = Request {
            req: &req,
        };

        let response = request.try_extract_email_from_jwt();
        match response {
            Ok(_email) => panic!("Should have been an error"),
            Err(e) => match e {
                Error::Forbidden(cause) => {
                    assert_eq!(cause, "No bearer scheme with jwt token".to_string())
                },
                _ => panic!("Wrong error"),
            }
        };
    }

    #[test]
    fn test_with_wrong_schema_in_header() {
        let jwt = jwt_testable();

        let req = actix_web::test::TestRequest::default()
            .insert_header((http::header::AUTHORIZATION, format!("OtherSchema {jwt}")))
            .to_srv_request();

        let request = Request {
            req: &req,
        };

        let response = request.try_extract_email_from_jwt();
        match response {
            Ok(_email) => panic!("Should have been an error"),
            Err(e) => match e {
                Error::Forbidden(cause) => {
                    assert_eq!(cause, "No bearer scheme with jwt token".to_string())
                },
                _ => panic!("Wrong error"),
            }
        };
    }

    #[test]
    fn test_with_no_jwt_in_header_with_correct_schema() {
        let req = actix_web::test::TestRequest::default()
            .insert_header((http::header::AUTHORIZATION, format!("Bearer ")))
            .to_srv_request();

        let request = Request {
            req: &req,
        };

        let response = request.try_extract_email_from_jwt();
        match response {
            Ok(_email) => panic!("Should have been an error"),
            Err(e) => match e {
                Error::Unauthorized(cause) => {
                    assert_eq!(cause, "No jwt token".to_string())
                },
                _ => panic!("Wrong error"),
            }
        };
    }
}