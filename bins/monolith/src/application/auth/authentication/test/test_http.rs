use actix_web::{web, test, http, ResponseError};
use error::Error;
use support::store::models::user::testable;
use crate::application::auth::authentication::{
    domain::Login, 
    data::LoginUserData,
    http::handle_login, 
    contract::{
        LoginContract, 
        MockPgRepositoryContract
    }, 
};

fn mock_service() -> impl LoginContract {
    let mut repository = MockPgRepositoryContract::new();
    repository
        .expect_get_user_by_email()
        .return_once( move |e|
            if e == "testmail@gmail.com" {
                Ok(testable(e, None, None, Some("correctPassword"), None))
            } else {
                Err(Error::Forbidden("user with given email does not exist".to_string()))
            }
        );

    Login {
        repository,
    }
}

#[test]
async fn test_handle_login_success() {
    let user_data = LoginUserData {
        email: Some("testmail@gmail.com".to_string()),
        password: Some("correctPassword".to_string()),
    };
    let data = web::Json(user_data);
    let request = test::TestRequest::post()
        .insert_header(("Host", "localhost"))
        .to_http_request();

    let response = handle_login(request, web::Data::new(mock_service()), data)
        .await
        .unwrap_or_else(|e| e.error_response());
    let response_token = response.headers().get("authorization").unwrap();
    
    assert_eq!(response.status(), http::StatusCode::OK);
    assert!(response.headers().contains_key("authorization") && !response_token.is_empty());
}

#[test]
async fn test_handle_login_fail_did_not_register() {
    let user_data = LoginUserData {
        email: Some("testmail_not_exist@gmail.com".to_string()),
        password: Some("correctPassword".to_string()),
    };
    let data = web::Json(user_data);
    let request = test::TestRequest::post()
        .insert_header(("Host", "localhost"))
        .to_http_request();

    let response = handle_login(request, web::Data::new(mock_service()), data)
        .await
        .unwrap_or_else(|e| e.error_response());
    
    assert_eq!(response.status(), http::StatusCode::FORBIDDEN);
}

#[test]
async fn test_handle_login_fail_incorrect_password() {
    let user_data = LoginUserData {
        email: Some("testmail@gmail.com".to_string()),
        password: Some("incorrectPassword".to_string()),
    };
    let data = web::Json(user_data);
    let request = test::TestRequest::post()
        .insert_header(("Host", "localhost"))
        .to_http_request();

    let response = handle_login(request, web::Data::new(mock_service()), data)
        .await
        .unwrap_or_else(|e| e.error_response());
    
    assert_eq!(response.status(), http::StatusCode::FORBIDDEN);
}

#[test]
async fn test_handle_fail_fail_is_not_an_email() {
    let user_data = LoginUserData { 
        email: Some("is_not_an_email".to_string()),
        password: Some("correctPassword".to_string()),
    };
    let data = web::Json(user_data);
    let request = test::TestRequest::post()
        .insert_header(("Host", "localhost"))
        .to_http_request();

    let response = handle_login(request, web::Data::new(mock_service()), data)
        .await
        .unwrap_or_else(|e| e.error_response());
    assert_eq!(response.status(), http::StatusCode::UNPROCESSABLE_ENTITY);
}

#[test]
async fn test_handle_fail_fail_no_email() {
    let user_data = LoginUserData { 
        email: None,
        password: Some("correctPassword".to_string()),
    };
    let data = web::Json(user_data);
    let request = test::TestRequest::post()
        .insert_header(("Host", "localhost"))
        .to_http_request();

    let response = handle_login(request, web::Data::new(mock_service()), data)
        .await
        .unwrap_or_else(|e| e.error_response());
    assert_eq!(response.status(), http::StatusCode::UNPROCESSABLE_ENTITY);
}

#[test]
async fn test_handle_fail_fail_no_password() {
    let user_data = LoginUserData { 
        email: Some("testmail@gmail.com".to_string()),
        password: None,
    };
    let data = web::Json(user_data);
    let request = test::TestRequest::post()
        .insert_header(("Host", "localhost"))
        .to_http_request();

    let response = handle_login(request, web::Data::new(mock_service()), data)
        .await
        .unwrap_or_else(|e| e.error_response());
    assert_eq!(response.status(), http::StatusCode::UNPROCESSABLE_ENTITY);
}

#[test]
async fn test_handle_fail_fail_no_email_and_password() {
    let user_data = LoginUserData { 
        email: None,
        password: None,
    };
    let data = web::Json(user_data);
    let request = test::TestRequest::post()
        .insert_header(("Host", "localhost"))
        .to_http_request();

    let response = handle_login(request, web::Data::new(mock_service()), data)
        .await
        .unwrap_or_else(|e| e.error_response());
    assert_eq!(response.status(), http::StatusCode::UNPROCESSABLE_ENTITY);
}