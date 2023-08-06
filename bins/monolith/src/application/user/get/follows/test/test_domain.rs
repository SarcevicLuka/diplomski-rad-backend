use crate::application::user::get::follows::{
    contract::{
        MockPgRepositoryContract, GetFollowsContract
    }, 
    domain::GetFollows, 
    data::UserFollowsAttributes, 
    test::data::{generate_testable_follows, generate_testable_followed_users}
};
use length_aware_paginator::Response;

#[actix_web::main]
#[test]
async fn test_get_followed_users_success() {
    let attributes = UserFollowsAttributes {
        page: Some(1),
        per_page: Some(10),
        sort: None,
        name: None,
    };

    let user_that_gets_follows = "test_user_id";
    let followed_users = generate_testable_followed_users(5);

    let follows = generate_testable_follows(5);

    let mut repository = MockPgRepositoryContract::new();
    repository
        .expect_get_followed_users()
        .return_once(move |_, _| Ok(Response {
            page: 1,
            per_page: 10,
            total: 5,
            last_page: 1,
            data: followed_users
        }));

    let obj = GetFollows {
        repository,
    };

    let response = 
        obj
            .get_followed_users(user_that_gets_follows, attributes)
            .await
            .unwrap();
    
    assert_eq!(response.data, follows.data);
}

#[actix_web::main]
#[test]
async fn test_get_following_users_success() {
    let attributes = UserFollowsAttributes {
        page: Some(1),
        per_page: Some(10),
        sort: None,
        name: None,
    };

    let user_that_gets_follows = "test_user_id";
    let following_users = generate_testable_followed_users(5);

    let following = generate_testable_follows(5);
    let test_following = following.clone();

    let mut repository = MockPgRepositoryContract::new();
    repository
        .expect_get_following_users()
        .return_once(move |_, _| Ok(Response {
            page: 1,
            per_page: 10,
            total: 5,
            last_page: 1,
            data: following_users
        }));

    let obj = GetFollows {
        repository,
    };

    let response = 
        obj
            .get_following_users(user_that_gets_follows, attributes)
            .await
            .unwrap();
    
    assert_eq!(response.data, test_following.data);
}