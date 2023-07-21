use error::Error;
use support::store::models::user::testable;
use crate::middleware::auth::contract::MockPgRepositoryContract;
use super::{
    contract::{
        MockRequestContract, 
        SessionContract
    }, 
    domain::Session
};

#[test]
fn test_with_valid_jwt_token_return_user_email() {
    let mut request = MockRequestContract::new();
    request
        .expect_try_extract_email_from_jwt()
        .return_once(move || Ok("validToken".to_string()));

    let mut repository = MockPgRepositoryContract::new();
    repository
        .expect_get_user_by_email()
        .return_once(move |_| {
            Ok(testable("test@test.com", None, None, None, None))
        });
    
    let mut service = Session {
        request,
        repository,
    };

    let response = match service.extract_valid_user() {
        Ok(user) => user.email,
        Err(e) => e.to_string(),
    };

    assert_eq!(response, "test@test.com".to_string());
}

#[test]
fn test_invalid_jwt_token() {
    let mut request = MockRequestContract::new();
    request
        .expect_try_extract_email_from_jwt()
        .return_once(move || Err(Error::Unauthorized("jwt token not valid".to_string())));

    let mut repository = MockPgRepositoryContract::new();
    repository
        .expect_get_user_by_email()
        .times(0);

    let mut service = Session {
        request,
        repository,
    };

    let response = match service.extract_valid_user() {
        Ok(user) => user.email,
        Err(e) => e.to_string(),
    };

    assert_eq!(response, "Authorization error: jwt token not valid".to_string());
}

#[test]
fn test_invalid_jwt_token_scheme() {
    let mut request = MockRequestContract::new();
    request
        .expect_try_extract_email_from_jwt()
        .return_once(move || Err(Error::Forbidden("no bearer scheme with jwt token".to_string())));

    let mut repository = MockPgRepositoryContract::new();
    repository
        .expect_get_user_by_email()
        .times(0);

    let mut service = Session {
        request,
        repository,
    };

    let response = match service.extract_valid_user() {
        Ok(user) => user.email,
        Err(e) => e.to_string(),
    };

    assert_eq!(response, "Forbidden error: no bearer scheme with jwt token".to_string());
}

#[test]
fn test_no_authorization_header() {
    let mut request = MockRequestContract::new();
    request
        .expect_try_extract_email_from_jwt()
        .return_once(move || Err(Error::Forbidden("no authorization header".to_string())));

    let mut repository = MockPgRepositoryContract::new();
    repository
        .expect_get_user_by_email()
        .times(0);

    let mut service = Session {
        request,
        repository,
    };

    let response = match service.extract_valid_user() {
        Ok(user) => user.email,
        Err(e) => e.to_string(),
    };

    assert_eq!(response, "Forbidden error: no authorization header".to_string());
}

#[test]
fn test_no_jwt_token_in_authorization_header() {
    let mut request = MockRequestContract::new();
    request
        .expect_try_extract_email_from_jwt()
        .return_once(move || Err(Error::Request("no jwt token in authorization header".to_string())));

    let mut repository = MockPgRepositoryContract::new();
    repository
        .expect_get_user_by_email()
        .times(0);

    let mut service = Session {
        request,
        repository,
    };

    let response = match service.extract_valid_user() {
        Ok(user) => user.email,
        Err(e) => e.to_string(),
    };

    assert_eq!(response, "Request error: no jwt token in authorization header".to_string());
}

#[test]
fn test_no_user_found_from_jwt_token() {
    let mut request = MockRequestContract::new();
    request
        .expect_try_extract_email_from_jwt()
        .return_once(move || Ok("validToken".to_string()));

    let mut repository = MockPgRepositoryContract::new();
    repository
        .expect_get_user_by_email()
        .return_once(move |_| Err(Error::Forbidden("user from jwt not found in database".to_string())));

    let mut service = Session {
        request,
        repository,
    };

    let response = match service.extract_valid_user() {
        Ok(user) => user.email,
        Err(e) => e.to_string(),
    };

    assert_eq!(response, "Forbidden error: user from jwt not found in database".to_string());
}