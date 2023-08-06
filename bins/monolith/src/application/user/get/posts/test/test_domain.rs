use crate::application::user::get::posts::{
    contract::{
        MockPgRepositoryContract, GetPostsContract
    }, 
    domain::GetPost, 
    data::UserPostsAttributes, 
    test::data::{generate_testable_posts, generate_testable_posts_and_likes}
};
use length_aware_paginator::Response;

#[actix_web::main]
#[test]
async fn test_get_followed_users_success() {
    let attributes = UserPostsAttributes {
        page: Some(1),
        per_page: Some(10),
        sort: None,
        name: None,
    };

    let user_id = "test_user_id";

    let posts_with_likes_union = generate_testable_posts_and_likes(5);
    let posts = generate_testable_posts(5);

    let mut repository = MockPgRepositoryContract::new();
    repository
        .expect_get_users_posts_paginated()
        .return_once(move |_, _| Ok(Response {
            page: 1,
            per_page: 10,
            total: 5,
            last_page: 1,
            data: posts_with_likes_union
        }));

    let obj = GetPost {
        repository,
    };

    let response = 
        obj
            .get_users_posts_paginated(user_id, attributes)
            .await
            .unwrap();
    
    assert_eq!(response.data, posts.data);
}