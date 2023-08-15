use crate::application::post::like::{
    contract::{
        MockPgRepositoryContract, LikePostContract
    }, 
    domain::LikePost
};
use support::store::models::post_like::testable;

#[actix_web::main]
#[test]
async fn test_like_post_success() {
    let user_id = "test_user_id";
    let post_id = "test_post_id";

    let mut repository = MockPgRepositoryContract::new();
    repository
        .expect_like_post()
        .return_once(move |user_id, post_id| Ok(
            testable(None, Some(user_id), Some(post_id))
        ));

    let obj = LikePost {
        repository,
    };

    let response = 
        obj
            .like_post(user_id, post_id)
            .await
            .unwrap();
    
    assert_eq!(response.user_id, user_id.to_string());
    assert_eq!(response.post_id, post_id.to_string());
}

#[actix_web::main]
#[test]
async fn test_remove_like_post_success() {
    let user_id = "test_user_id";
    let post_id = "test_post_id";

    let mut repository = MockPgRepositoryContract::new();
    repository
        .expect_remove_like_post()
        .return_once(move |_user_id, _post_id| Ok(1));

    let obj = LikePost {
        repository,
    };

    let response = 
        obj
            .remove_like_post(user_id, post_id)
            .await
            .unwrap();
    
    assert_eq!(response, ());
}