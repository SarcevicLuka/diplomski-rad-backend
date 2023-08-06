use actix_web::{ResponseError, test, HttpMessage, web, http};
use error::Error;
use support::store::models::{
    user::{testable as user_testable, DisplayUser},
    user_follow::testable};
use crate::application::user::follow::{
    contract::{
        FollowUserContract, 
        MockPgRepositoryContract
    }, 
    domain::FollowUser, 
    http::{
        http_follow::handle_follow_user, 
        http_unfollow::handle_unfollow_user
    }
};

fn mock_service() -> impl FollowUserContract {
    let mut repository = MockPgRepositoryContract::new();
    repository
        .expect_follow_user()
        .return_once(move |request_user_id, response_user_id| {
            if request_user_id == "test_request_user_id" && response_user_id == "test_response_user_id" {
                Ok(
                    testable(request_user_id, response_user_id)
                )
            } else if request_user_id == "test_request_user_id" && response_user_id == "test_request_user_id" {
                Err(Error::Request("Cannot follow yourself".to_string()))
            } else {
                Err(Error::Request("No user found with given id".to_string()))
            }
        });

    repository
        .expect_unfollow_user()
        .return_once(move |request_user_id, response_user_id| {
            if request_user_id == "test_request_user_id" && response_user_id == "test_response_user_id" {
                Ok(1)
            } else if request_user_id == "test_request_user_id" && response_user_id == "test_request_user_id" {
                Err(Error::Request("Cannot unfollow yourself".to_string()))
            } else {
                Err(Error::Request("Ids are not valid".to_string()))
            }
        });

    FollowUser {
        repository,
    }
}

#[test]
async fn test_handle_follow_user_success() {
    let user = user_testable("test@test.com", None, None, None, Some("test_request_user_id"));
    let auth_user = DisplayUser::from(user);
 
    let request = test::TestRequest::post()
        .param("user_id", "test_response_user_id")
        .to_http_request();
    request.extensions_mut().insert(auth_user);

    let response = 
        handle_follow_user(request, web::Data::new(mock_service()))
            .await
            .unwrap_or_else(|e| e.error_response());
    assert_eq!(response.status(), http::StatusCode::OK);
}

#[test]
async fn test_handle_follow_user_fail_same_id() {
    let user = user_testable("test@test.com", None, None, None, Some("test_request_user_id"));
    let auth_user = DisplayUser::from(user);
 
    let request = test::TestRequest::post()
        .param("user_id", "test_request_user_id")
        .to_http_request();
    request.extensions_mut().insert(auth_user);

    let response = 
        handle_follow_user(request, web::Data::new(mock_service()))
            .await
            .unwrap_or_else(|e| e.error_response());
    assert_eq!(response.status(), http::StatusCode::BAD_REQUEST);
}

#[test]
async fn test_handle_follow_user_fail_non_existent_id() {
    let user = user_testable("test@test.com", None, None, None, Some("test_request_user_id"));
    let auth_user = DisplayUser::from(user);
 
    let request = test::TestRequest::post()
        .param("user_id", "test_request_user_id_non_existent")
        .to_http_request();
    request.extensions_mut().insert(auth_user);

    let response = 
        handle_follow_user(request, web::Data::new(mock_service()))
            .await
            .unwrap_or_else(|e| e.error_response());
    assert_eq!(response.status(), http::StatusCode::BAD_REQUEST);
}

#[test]
async fn test_handle_unfollow_user_success() {
    let user = user_testable("test@test.com", None, None, None, Some("test_request_user_id"));
    let auth_user = DisplayUser::from(user);
 
    let request = test::TestRequest::post()
        .param("user_id", "test_response_user_id")
        .to_http_request();
    request.extensions_mut().insert(auth_user);

    let response = 
        handle_unfollow_user(request, web::Data::new(mock_service()))
            .await
            .unwrap_or_else(|e| e.error_response());
    assert_eq!(response.status(), http::StatusCode::OK);
}

#[test]
async fn test_handle_unfollow_user_fail_same_id() {
    let user = user_testable("test@test.com", None, None, None, Some("test_request_user_id"));
    let auth_user = DisplayUser::from(user);
 
    let request = test::TestRequest::post()
        .param("user_id", "test_request_user_id")
        .to_http_request();
    request.extensions_mut().insert(auth_user);

    let response = 
        handle_unfollow_user(request, web::Data::new(mock_service()))
            .await
            .unwrap_or_else(|e| e.error_response());
    assert_eq!(response.status(), http::StatusCode::BAD_REQUEST);
}

#[test]
async fn test_handle_unfollow_user_fail_non_existent_id() {
    let user = user_testable("test@test.com", None, None, None, Some("test_request_user_id"));
    let auth_user = DisplayUser::from(user);
 
    let request = test::TestRequest::post()
        .param("user_id", "test_request_user_id_non_existent")
        .to_http_request();
    request.extensions_mut().insert(auth_user);

    let response = 
        handle_unfollow_user(request, web::Data::new(mock_service()))
            .await
            .unwrap_or_else(|e| e.error_response());
    assert_eq!(response.status(), http::StatusCode::BAD_REQUEST);
}