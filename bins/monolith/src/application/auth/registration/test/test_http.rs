// use actix_web::{web, test, http, ResponseError};
// use support::store::models::user;
// use crate::application::auth::registration::{
//     data::RegistrationUserData, 
//     http::handle_register, 
//     domain::Registration,
//     contract::{
//         RegistrationContract, 
//         MockPgRepositoryContract, 
//         MockPgServiceContract
//     }
// };

// fn mock_service() -> impl RegistrationContract {
//     let mut repository = MockPgRepositoryContract::new();
//     repository
//         .expect_exists()
//         .return_once(move |e| {
//             if e == "testmail@gmail.com" {
//                 true
//             } else {
//                 false
//             }
//         });
    
//     let mut service = MockPgServiceContract::new();
//     service
//         .expect_create_user()
//         .return_once(|_d| Ok(
//             user::testable(
//                 "test@test.com", 
//                 None, 
//                 None, 
//                 Some("test"),
//                 None,
//             )
//         ));

//     Registration {
//         repository,
//         service,
//     }
// }

// #[test]
// async fn test_handle_register_success() {
//     let user_data = RegistrationUserData { 
//         email: Some("testmail_not_exist@gmail.com".to_string()), 
//         first_name: Some("John".to_string()), 
//         last_name: Some("Doe".to_string()), 
//         password: Some("password that is not Weak!1".to_string()),
//     };
//     let data = web::Json(user_data);
//     let request = test::TestRequest::post()
//         .insert_header(("Host", "localhost"))
//         .to_http_request();

//     let response = handle_register(request, web::Data::new(mock_service()), data)
//         .await
//         .unwrap_or_else(|e| e.error_response());
//     let response_token = response.headers().get("authorization").unwrap();
    
//     assert_eq!(response.status(), http::StatusCode::CREATED);
//     assert!(response.headers().contains_key("authorization") && !response_token.is_empty());
// }

// #[test]
// async fn test_handle_register_fail_email_already_exists() {
//     let user_data = RegistrationUserData { 
//         email: Some("testmail@gmail.com".to_string()), 
//         first_name: Some("John".to_string()), 
//         last_name: Some("Doe".to_string()), 
//         password: Some("password that is not Weak!1".to_string()),
//     };
//     let data = web::Json(user_data);
//     let request = test::TestRequest::post()
//         .insert_header(("Host", "localhost"))
//         .to_http_request();

//     let response = handle_register(request, web::Data::new(mock_service()), data)
//         .await
//         .unwrap_or_else(|e| e.error_response());
//     assert_eq!(response.status(), http::StatusCode::BAD_REQUEST);
// }

// #[test]
// async fn test_handle_register_fail_incorrect_email() {
//     let user_data = RegistrationUserData { 
//         email: Some("testmailmail.com".to_string()), 
//         first_name: Some("John".to_string()), 
//         last_name: Some("Doe".to_string()), 
//         password: Some("password that is not Weak!1".to_string()),
//     };
//     let data = web::Json(user_data);
//     let request = test::TestRequest::post()
//         .insert_header(("Host", "localhost"))
//         .to_http_request();

//     let response = handle_register(request, web::Data::new(mock_service()), data)
//         .await
//         .unwrap_or_else(|e| e.error_response());
//     assert_eq!(response.status(), http::StatusCode::UNPROCESSABLE_ENTITY);
// }

// #[test]
// async fn test_handle_register_fail_no_email() {
//     let user_data = RegistrationUserData { 
//         email: None, 
//         first_name: Some("John".to_string()), 
//         last_name: Some("Doe".to_string()), 
//         password: Some("password that is not Weak!1".to_string()),
//     };
//     let data = web::Json(user_data);
//     let request = test::TestRequest::post()
//         .insert_header(("Host", "localhost"))
//         .to_http_request();

//     let response = handle_register(request, web::Data::new(mock_service()), data)
//         .await
//         .unwrap_or_else(|e| e.error_response());
//     assert_eq!(response.status(), http::StatusCode::UNPROCESSABLE_ENTITY);
// }

// #[test]
// async fn test_handle_register_fail_no_password() {
//     let user_data = RegistrationUserData { 
//         email: Some("testmail@gmail.com".to_string()), 
//         first_name: Some("John".to_string()), 
//         last_name: Some("Doe".to_string()), 
//         password: None,
//     };
//     let data = web::Json(user_data);
//     let request = test::TestRequest::post()
//         .insert_header(("Host", "localhost"))
//         .to_http_request();

//     let response = handle_register(request, web::Data::new(mock_service()), data)
//         .await
//         .unwrap_or_else(|e| e.error_response());
//     assert_eq!(response.status(), http::StatusCode::UNPROCESSABLE_ENTITY);
// }

// #[test]
// async fn test_handle_register_fail_weak_password() {
//     let user_data = RegistrationUserData { 
//         email: Some("testmail@gmail.com".to_string()), 
//         first_name: Some("John".to_string()), 
//         last_name: Some("Doe".to_string()), 
//         password: Some("weak_password".to_string()),
//     };
//     let data = web::Json(user_data);
//     let request = test::TestRequest::post()
//         .insert_header(("Host", "localhost"))
//         .to_http_request();

//     let response = handle_register(request, web::Data::new(mock_service()), data)
//         .await
//         .unwrap_or_else(|e| e.error_response());
//     assert_eq!(response.status(), http::StatusCode::UNPROCESSABLE_ENTITY);
// }

// #[test]
// async fn test_handle_register_fail_no_first_name() {
//     let user_data = RegistrationUserData { 
//         email: Some("testmail@gmail.com".to_string()), 
//         first_name: None, 
//         last_name: Some("Doe".to_string()), 
//         password: Some("weak_password".to_string()),
//     };
//     let data = web::Json(user_data);
//     let request = test::TestRequest::post()
//         .insert_header(("Host", "localhost"))
//         .to_http_request();

//     let response = handle_register(request, web::Data::new(mock_service()), data)
//         .await
//         .unwrap_or_else(|e| e.error_response());
//     assert_eq!(response.status(), http::StatusCode::UNPROCESSABLE_ENTITY);
// }

// #[test]
// async fn test_handle_register_fail_first_name_too_short() {
//     let user_data = RegistrationUserData { 
//         email: Some("testmail@gmail.com".to_string()), 
//         first_name: Some("L".to_string()), 
//         last_name: Some("Doe".to_string()), 
//         password: Some("weak_password".to_string()),
//     };
//     let data = web::Json(user_data);
//     let request = test::TestRequest::post()
//         .insert_header(("Host", "localhost"))
//         .to_http_request();

//     let response = handle_register(request, web::Data::new(mock_service()), data)
//         .await
//         .unwrap_or_else(|e| e.error_response());
//     assert_eq!(response.status(), http::StatusCode::UNPROCESSABLE_ENTITY);
// }

// #[test]
// async fn test_handle_register_fail_first_name_too_long() {
//     let user_data = RegistrationUserData { 
//         email: Some("testmail@gmail.com".to_string()), 
//         first_name: Some("___First_name_too_long_Number_of_characters_is_51__".to_string()), 
//         last_name: Some("Doe".to_string()), 
//         password: Some("weak_password".to_string()),
//     };
//     let data = web::Json(user_data);
//     let request = test::TestRequest::post()
//         .insert_header(("Host", "localhost"))
//         .to_http_request();

//     let response = handle_register(request, web::Data::new(mock_service()), data)
//         .await
//         .unwrap_or_else(|e| e.error_response());
//     assert_eq!(response.status(), http::StatusCode::UNPROCESSABLE_ENTITY);
// }

// #[test]
// async fn test_handle_register_fail_no_last_name() {
//     let user_data = RegistrationUserData { 
//         email: Some("testmail@gmail.com".to_string()), 
//         first_name: Some("John".to_string()), 
//         last_name: None, 
//         password: Some("weak_password".to_string()),
//     };
//     let data = web::Json(user_data);
//     let request = test::TestRequest::post()
//         .insert_header(("Host", "localhost"))
//         .to_http_request();

//     let response = handle_register(request, web::Data::new(mock_service()), data)
//         .await
//         .unwrap_or_else(|e| e.error_response());
//     assert_eq!(response.status(), http::StatusCode::UNPROCESSABLE_ENTITY);
// }

// #[test]
// async fn test_handle_register_fail_last_name_too_short() {
//     let user_data = RegistrationUserData { 
//         email: Some("testmail@gmail.com".to_string()), 
//         first_name: Some("John".to_string()), 
//         last_name: Some("D".to_string()), 
//         password: Some("weak_password".to_string()),
//     };
//     let data = web::Json(user_data);
//     let request = test::TestRequest::post()
//         .insert_header(("Host", "localhost"))
//         .to_http_request();

//     let response = handle_register(request, web::Data::new(mock_service()), data)
//         .await
//         .unwrap_or_else(|e| e.error_response());
//     assert_eq!(response.status(), http::StatusCode::UNPROCESSABLE_ENTITY);
// }

// #[test]
// async fn test_handle_register_fail_last_name_too_long() {
//     let user_data = RegistrationUserData { 
//         email: Some("testmail@gmail.com".to_string()), 
//         first_name: Some("John".to_string()), 
//         last_name: Some("___First_name_too_long_Number_of_characters_is_51__".to_string()), 
//         password: Some("weak_password".to_string()),
//     };
//     let data = web::Json(user_data);
//     let request = test::TestRequest::post()
//         .insert_header(("Host", "localhost"))
//         .to_http_request();

//     let response = handle_register(request, web::Data::new(mock_service()), data)
//         .await
//         .unwrap_or_else(|e| e.error_response());
//     assert_eq!(response.status(), http::StatusCode::UNPROCESSABLE_ENTITY);
// }