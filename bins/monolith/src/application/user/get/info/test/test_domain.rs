// use crate::application::user::get::info::{
//     contract::{
//         MockPgRepositoryContract, GetUserContract
//     }, 
//     domain::GetUser
// };
// use diesel::result::Error as DieselError;
// use error::Error;
// use support::store::models::user::testable;

// #[actix_web::main]
// #[test]
// async fn test_get_user_success() {
//     let test_user_id = "user_requesting_test_id";

//     let mut repository = MockPgRepositoryContract::new();
//     repository
//         .expect_get_user_by_id()
//         .return_once(move |user_id| Ok(
//             testable("email@gmail.com", Some("first_name"), Some("last_name"), Some("password"), Some(user_id)).into()
//         ));

//     let obj = GetUser {
//         repository,
//     };

//     let response = 
//         obj
//             .get_user_by_id(test_user_id)
//             .await
//             .unwrap();
    
//     assert_eq!(response.id, test_user_id);
// }

// #[actix_web::main]
// #[test]
// async fn test_get_user_fail_non_existent_id() {
//     let test_user_id = "user_requesting_test_id";

//     let mut repository = MockPgRepositoryContract::new();
//     repository
//         .expect_get_user_by_id()
//         .return_once(move |_user_id| 
//             Err(Error::Diesel(DieselError::NotFound))    
//         );
//     let obj = GetUser {
//         repository,
//     };

//     let response = 
//         obj
//             .get_user_by_id(test_user_id)
//             .await;
    
//     match response {
//         Ok(_user) => panic!("Should have been error"),
//         Err(e) => match e {
//             Error::Diesel(DieselError::NotFound) => {},
//             _ => panic!("Wrong error recieved"),
//         },
//     };
// }