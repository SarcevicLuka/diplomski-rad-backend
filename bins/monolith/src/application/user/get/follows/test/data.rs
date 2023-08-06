use support::store::models::user::{DisplayUser, testable, User};

use crate::application::user::get::follows::data::PaginatedFollowsResponse;

#[allow(dead_code)]
pub fn generate_testable_follows(num_of_follows: i32) -> PaginatedFollowsResponse {
    let last_page = (num_of_follows / 10) as f64;
    let last_page = last_page.ceil() as i64;
    let mut follows = 
        PaginatedFollowsResponse {
            page: 1,
            per_page: 10,
            total: num_of_follows as i64,
            last_page,
            data: vec![], 
        };
    
    for id in 0..num_of_follows {
        follows.data.push(
            DisplayUser::from(testable(
                "email@gmail.com", 
            Some("first_name"), 
            Some("last_name"), 
            Some("password"),
            Some(&id.to_string())
            ))
        )
    };

    follows
}

#[allow(dead_code)]
pub fn generate_testable_followed_users(num_of_follows: i32) -> Vec<User> {
    let mut users: Vec<User> = vec![];

    for id in 0..num_of_follows {
        users.push(
            testable(
                "email@gmail.com", 
            Some("first_name"), 
            Some("last_name"), 
            Some("password"),
            Some(&id.to_string())
            )
        )
    };

    users
}