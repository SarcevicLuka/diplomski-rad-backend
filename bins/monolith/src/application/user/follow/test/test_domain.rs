use crate::application::user::follow::{
    contract::{
        MockPgRepositoryContract, FollowUserContract
    }, 
    domain::FollowUser
};
use error::Error;
use support::store::models::user_follow::testable;

#[actix_web::main]
#[test]
async fn test_follow_user_success() {
    let request_user_id = "user_requesting_test_id";
    let response_user_id = "user_responding_test_id";

    let mut repository = MockPgRepositoryContract::new();
    repository
        .expect_follow_user()
        .return_once(move |user_requesting, user_responding| Ok(
            testable(user_requesting, user_responding)
        ));

    let obj = FollowUser {
        repository,
    };

    let response = 
        obj
            .follow_user(request_user_id, response_user_id)
            .await
            .unwrap();
    
    assert_eq!(response.user_requesting, request_user_id);
    assert_eq!(response.user_responding, response_user_id);
}

#[actix_web::main]
#[test]
async fn test_follow_user_fail_follow_yourself() {
    let request_user_id = "user_requesting_test_id";
    let response_user_id = "user_requesting_test_id";

    let mut repository = MockPgRepositoryContract::new();
    repository
        .expect_follow_user()
        .return_once(move |user_requesting, user_responding| Ok(
            testable(user_requesting, user_responding)
        ));

    let obj = FollowUser {
        repository,
    };

    let response = 
        obj
            .follow_user(request_user_id, response_user_id)
            .await;
    
    match response {
        Ok(_user_follow) => panic!("Should have been error"),
        Err(e) => match e {
            Error::Request(cause) => {
                assert_eq!(cause, "Cannot follow yourself".to_string())
            }
            _ => panic!("Wrong error recieved"),
        }
    };
}

#[actix_web::main]
#[test]
async fn test_follow_user_fail_follow_non_existent_user() {
    let request_user_id = "user_requesting_test_id";
    let response_user_id = "non_existent_test_id";

    let mut repository = MockPgRepositoryContract::new();
    repository
        .expect_follow_user()
        .return_once(move |_, _| 
            Err(Error::Request("No user found with given id".to_string()))
        );

    let obj = FollowUser {
        repository,
    };

    let response = 
        obj
            .follow_user(request_user_id, response_user_id)
            .await;
    
    match response {
        Ok(_user_follow) => panic!("Should have been error"),
        Err(e) => match e {
            Error::Request(cause) => {
                assert_eq!(cause, "No user found with given id".to_string())
            }
            _ => panic!("Wrong error recieved"),
        }
    };
}

#[actix_web::main]
#[test]
async fn test_unfollow_user_success() {
    let request_user_id = "user_requesting_test_id";
    let response_user_id = "user_responding_test_id";

    let mut repository = MockPgRepositoryContract::new();
    repository
        .expect_unfollow_user()
        .return_once(move |_, _| Ok(1));

    let obj = FollowUser {
        repository,
    };

    let response = 
        obj
            .unfollow_user(request_user_id, response_user_id)
            .await
            .unwrap();
    
    assert_eq!(response, ());
}

#[actix_web::main]
#[test]
async fn test_unfollow_user_fail_unfollow_yourself() {
    let request_user_id = "user_requesting_test_id";
    let response_user_id = "user_requesting_test_id";

    let mut repository = MockPgRepositoryContract::new();
    repository
        .expect_unfollow_user()
        .return_once(move |_, _| Ok(0));

    let obj = FollowUser {
        repository,
    };

    let response = 
        obj
            .unfollow_user(request_user_id, response_user_id)
            .await;
    
    match response {
        Ok(_user_unfollow) => panic!("Should have been error"),
        Err(e) => match e {
            Error::Request(cause) => {
                assert_eq!(cause, "Cannot unfollow yourself".to_string())
            }
            _ => panic!("Wrong error recieved"),
        }
    };
}

#[actix_web::main]
#[test]
async fn test_unfollow_user_fail_unfollow_non_existent_user() {
    let request_user_id = "user_requesting_test_id";
    let response_user_id = "non_existent_test_id";

    let mut repository = MockPgRepositoryContract::new();
    repository
        .expect_unfollow_user()
        .return_once(move |_, _| Ok(0));

    let obj = FollowUser {
        repository,
    };

    let response = 
        obj
            .unfollow_user(request_user_id, response_user_id)
            .await;
    
    match response {
        Ok(_user_unfollow) => panic!("Should have been error"),
        Err(e) => match e {
            Error::Request(cause) => {
                assert_eq!(cause, "Ids are not valid".to_string())
            }
            _ => panic!("Wrong error recieved"),
        }
    };
}