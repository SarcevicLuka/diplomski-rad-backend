// use actix_web::{ResponseError, test, web, http};
// use error::Error;
// use diesel::result::Error as DieselError;
// use support::store::models::user::testable;
// use crate::application::user::get::info::{
//     contract::{
//         MockPgRepositoryContract, GetUserContract
//     }, 
//     domain::GetUser, 
//     http::handle_get_user
// };

// fn mock_service() -> impl GetUserContract {
//     let mut repository = MockPgRepositoryContract::new();
//     repository
//         .expect_get_user_by_id()
//         .return_once(move |user_id| {
//             if user_id == "test_user_id" {
//                 Ok(
//                     testable("email@gmail.com", Some("first_name"), Some("last_name"), Some("password"), Some(user_id)).into()
//                 )
//             } else {
//                 Err(Error::Diesel(DieselError::NotFound))
//             }
//         });

//     GetUser {
//         repository,
//     }
// }

// #[test]
// async fn test_handle_get_user_info_success() {
//     let request = test::TestRequest::post()
//         .param("user_id", "test_user_id")
//         .to_http_request();

//     let response = 
//         handle_get_user(request, web::Data::new(mock_service()))
//             .await
//             .unwrap_or_else(|e| e.error_response());
        
//     assert_eq!(response.status(), http::StatusCode::OK);
// }

// #[test]
// async fn test_handle_get_following_users_success() {
//     let request = test::TestRequest::post()
//         .param("user_id", "test_user_id_non_existent")
//         .to_http_request();

//     let response = 
//         handle_get_user(request, web::Data::new(mock_service()))
//             .await
//             .unwrap_or_else(|e| e.error_response());
        
//     assert_eq!(response.status(), http::StatusCode::INTERNAL_SERVER_ERROR);
// }