use support::store::models::post::testable;
use crate::application::post::create::{
    contract::{
        MockPgRepositoryContract, 
        CreatePostContract
    }, 
    domain::CreatePost
};
use super::data::{generate_create_post_data, generate_create_watch_data};

#[actix_web::main]
#[test]
async fn test_create_post_success() {
    let watch_data = generate_create_watch_data();
    let post_data = generate_create_post_data(Some(watch_data), Some("review"), Some(5));

    let mut repository = MockPgRepositoryContract::new();

    repository
        .expect_create_watch()
        .return_once(move |_watch_data| Ok(
            String::from("test_watch_id")
        ));

    repository
        .expect_create_post()
        .return_once(move |post_data| Ok(
            testable(
                None, 
                Some(&post_data.user_id), 
                Some(&post_data.watch_id), 
                Some(&post_data.review), 
                Some(post_data.score)
            )
        ));

    let obj = CreatePost {
        repository    
    };

    let response = obj.create_post("test_user_id", post_data).await.unwrap();
    assert_eq!(response.user_id, "test_user_id");
    assert_eq!(response.watch_id, "test_watch_id");
}