use actix_web::{ResponseError, test, web, http};
use error::Error;
use crate::application::user::get::follows::{
    contract::{
        MockPgRepositoryContract, GetFollowsContract
    }, 
    domain::GetFollows, 
    data::UserFollowsAttributes,
    test::data::generate_testable_followed_users, 
    http::{http_get_follows::handle_get_follows, http_get_following::handle_get_following}
};
use length_aware_paginator::Response;

fn mock_service() -> impl GetFollowsContract {
    let follows = generate_testable_followed_users(5);
    let followed = follows.clone();

    let mut repository = MockPgRepositoryContract::new();
    repository
        .expect_get_followed_users()
        .return_once(move |user_id, attributes| {
            if user_id == "test_user_id" {
                Ok(
                    Response {
                        page: attributes.page.unwrap(),
                        per_page: attributes.per_page.unwrap(),
                        total: 5,
                        last_page: 1,
                        data: follows
                    }
                )
            } else {
                Err(Error::Request("No user found with given id".to_string()))
            }
        });

    repository
        .expect_get_following_users()
        .return_once(move |user_id, attributes| {
            if user_id == "test_user_id" {
                Ok(
                    Response {
                        page: attributes.page.unwrap(),
                        per_page: attributes.per_page.unwrap(),
                        total: 5,
                        last_page: 1,
                        data: followed
                    }
                )
            } else {
                Err(Error::Request("No user found with given id".to_string()))
            }
        });

    GetFollows {
        repository,
    }
}

#[test]
async fn test_handle_get_followed_users_success() {
    let attributes = UserFollowsAttributes {
        page: Some(1),
        per_page: Some(10),
        sort: None,
        name: None,
    };
    let request = test::TestRequest::post()
        .param("user_id", "test_user_id")
        .to_http_request();

    let response = 
        handle_get_follows(request, web::Query(attributes), web::Data::new(mock_service()))
            .await
            .unwrap_or_else(|e| e.error_response());
        
    assert_eq!(response.status(), http::StatusCode::OK);
}

#[test]
async fn test_handle_get_following_users_success() {
    let attributes = UserFollowsAttributes {
        page: Some(1),
        per_page: Some(10),
        sort: None,
        name: None,
    };
    let request = test::TestRequest::post()
        .param("user_id", "test_user_id")
        .to_http_request();

    let response = 
        handle_get_following(request, web::Query(attributes), web::Data::new(mock_service()))
            .await
            .unwrap_or_else(|e| e.error_response());
        
    assert_eq!(response.status(), http::StatusCode::OK);
}