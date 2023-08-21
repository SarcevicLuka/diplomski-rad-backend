// use support::store::models::post::{testable, Post};

// use crate::application::user::get::posts::data::{PaginatedUsersPostsResponse, PostWithLikes};

// #[allow(dead_code)]
// pub fn generate_testable_posts(num_of_follows: i32) -> PaginatedUsersPostsResponse {
//     let last_page = (num_of_follows / 10) as f64;
//     let last_page = last_page.ceil() as i64;
//     let mut posts = 
//         PaginatedUsersPostsResponse {
//             page: 1,
//             per_page: 10,
//             total: num_of_follows as i64,
//             last_page,
//             data: vec![], 
//         };
    
//     for id in 0..num_of_follows {
//         posts.data.push(
//             PostWithLikes {
//                 num_likes: 4,
//                 post: testable(
//                         Some(&id.to_string()), 
//                         Some("test_user_id"), 
//                         Some("test_watch_id"), 
//                         Some("review"),
//                          Some(5)
//                     )
//             }
//         )
//     };

//     posts
// }

// #[allow(dead_code)]
// pub fn generate_testable_posts_and_likes(num_of_follows: i32) -> Vec<(Post, i64)> {
//     let mut posts_with_likes: Vec<(Post, i64)> = vec![];
    
//     for id in 0..num_of_follows {
//         posts_with_likes.push(
//             (
//                 testable(
//                     Some(&id.to_string()), 
//                     Some("test_user_id"), 
//                     Some("test_watch_id"), 
//                     Some("review"),
//                      Some(5)
//                 ),
//                 4
//             )
//         )
//     };

//     posts_with_likes
// }