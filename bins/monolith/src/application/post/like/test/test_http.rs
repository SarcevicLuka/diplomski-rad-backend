use actix_web::{ResponseError, test, HttpMessage, web, http};
use support::store::models::{
    post_like::testable, 
    user::{testable as user_testable, DisplayUser}
};
use crate::application::post::like::{
    contract::{
        LikePostContract, 
        MockPgRepositoryContract
    }, 
    domain::LikePost, 
    http::{
        http_like::handle_like_post, 
        http_remove_like::handle_remove_like_post
    }
};

fn mock_service() -> impl LikePostContract {
    let mut repository = MockPgRepositoryContract::new();
    repository
        .expect_like_post()
        .return_once(move |user_id, post_id| {
            Ok(
                testable(None, Some(user_id), Some(post_id))
            )
        });

    repository
        .expect_remove_like_post()
        .return_once(move |_user_id, _post_id| {
                Ok(1)
        });

    LikePost {
        repository,
    }
}

#[test]
async fn test_handle_like_post_success() {
    let user = user_testable("test@test.com", None, None, None, Some("test_user_id"));
    let auth_user = DisplayUser::from(user);
 
    let request = test::TestRequest::post()
        .param("post_id", "test_post_id")
        .to_http_request();
    request.extensions_mut().insert(auth_user);

    let response = 
        handle_like_post(request, web::Data::new(mock_service()))
            .await
            .unwrap_or_else(|e| e.error_response());
    assert_eq!(response.status(), http::StatusCode::OK);
}

#[test]
async fn test_handle_remove_like_post_success() {
    let user = user_testable("test@test.com", None, None, None, Some("test_user_id"));
    let auth_user = DisplayUser::from(user);
 
    let request = test::TestRequest::post()
        .param("post_id", "test_post_id")
        .to_http_request();
    request.extensions_mut().insert(auth_user);

    let response = 
        handle_remove_like_post(request, web::Data::new(mock_service()))
            .await
            .unwrap_or_else(|e| e.error_response());
    assert_eq!(response.status(), http::StatusCode::OK);
}