use actix_web::{ResponseError, test, web, http};
use error::Error;
use crate::application::user::get::posts::{
    contract::{
        MockPgRepositoryContract, GetPostsContract
    }, 
    domain::GetPost, 
    test::data::generate_testable_posts_and_likes, 
    http::handle_get_users_posts, data::UserPostsAttributes
};
use length_aware_paginator::Response;

fn mock_service() -> impl GetPostsContract {
    let posts_with_likes_union = generate_testable_posts_and_likes(5);

    let mut repository = MockPgRepositoryContract::new();
    repository
        .expect_get_users_posts_paginated()
        .return_once(move |user_id, attributes| {
            if user_id == "test_user_id" {
                Ok(
                    Response {
                        page: attributes.page.unwrap(),
                        per_page: attributes.per_page.unwrap(),
                        total: 5,
                        last_page: 1,
                        data: posts_with_likes_union
                    }
                )
            } else {
                Err(Error::Request("No user found with given id".to_string()))
            }
        });

    GetPost {
        repository,
    }
}

#[test]
async fn test_handle_get_users_posts_success() {
    let attributes = UserPostsAttributes {
        page: Some(1),
        per_page: Some(10),
        sort: None,
        name: None,
    };
    let request = test::TestRequest::post()
        .param("user_id", "test_user_id")
        .to_http_request();

    let response = 
        handle_get_users_posts(request, web::Query(attributes), web::Data::new(mock_service()))
            .await
            .unwrap_or_else(|e| e.error_response());
        
    assert_eq!(response.status(), http::StatusCode::OK);
}