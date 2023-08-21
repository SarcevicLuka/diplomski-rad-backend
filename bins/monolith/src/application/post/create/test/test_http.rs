// use actix_web::{web, test, http, ResponseError, HttpMessage};
// use support::store::models::{
//     user::{ 
//         testable as user_testable, 
//         DisplayUser
//     }, 
//     post::testable
// };
// use crate::application::post::create::{
//     http::handle_create_post, 
//     domain::CreatePost,
//     contract::{
//         CreatePostContract, 
//         MockPgRepositoryContract, 
//     },
//     test::data::{generate_create_watch_data, generate_create_post_data}
// };
// fn mock_service() -> impl CreatePostContract {
//     let mut repository = MockPgRepositoryContract::new();

//     repository
//         .expect_create_watch()
//         .return_once(move |_watch_data| Ok(
//             String::from("test_watch_id")
//         ));

//     repository
//         .expect_create_post()
//         .return_once(move |post_data| Ok(
//             testable(
//                 None, 
//                 Some(&post_data.user_id), 
//                 Some(&post_data.watch_id), 
//                 Some(&post_data.review), 
//                 Some(post_data.score)
//             )
//         ));

//     CreatePost {
//         repository
//     }
// }

// #[test]
// async fn test_handle_create_post_success() {
//     let user = user_testable("test@test.com", None, None, None, Some("test_user_id"));
//     let auth_user = DisplayUser::from(user);
//     let watch_data = generate_create_watch_data();
//     let post_data = generate_create_post_data(Some(watch_data), Some("review"), Some(5));
 
//     let request = test::TestRequest::post()
//         .to_http_request();
//     request.extensions_mut().insert(auth_user);

//     let response = 
//         handle_create_post(request, web::Data::new(mock_service()), web::Json(post_data))
//             .await
//             .unwrap_or_else(|e| e.error_response());
//     assert_eq!(response.status(), http::StatusCode::CREATED);
// }