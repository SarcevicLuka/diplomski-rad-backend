use support::store::models::user::testable;
use error::Error;
use crate::application::auth::registration::{
    data::RegistrationUserData, 
    contract::{
        MockPgRepositoryContract, 
        MockPgServiceContract, RegistrationContract
    }, domain::Registration
};

#[actix_web::main]
#[test]
async fn test_user_registration_success() {
    let user_data = create_registration_data();

    let mut repository = MockPgRepositoryContract::new();
    let mut service = MockPgServiceContract::new();

    repository
        .expect_exists()
        .return_const(false);

    service
        .expect_create_user()
        .return_once(move |r| Ok(testable(&r.email, None, None, None, None)));

    let obj = Registration {
        repository,
        service,
    };

    let (authenticated_user, _token) = obj.register(user_data).await.unwrap();
    assert_eq!(&authenticated_user.email, "testmail@gmail.com");
}

#[actix_web::main]
#[test]
async fn test_user_registration_fail_mail_in_use() {
    let user_data = create_registration_data();

    let mut repository = MockPgRepositoryContract::new();
    let service = MockPgServiceContract::new();

    repository
        .expect_exists()
        .return_const(true);

    let obj = Registration {
        repository,
        service,
    };

    let response = obj.register(user_data).await;
    
    match response {
        Ok(_) => panic!("Should have been error"),
        Err(e) => match e {
            Error::Request(cause) => {
                assert_eq!(cause, "Email is already in use".to_string())
            }
            _ => panic!("Wrong error recieved"),
        }
    };
}

#[actix_web::main]
#[test]
async fn test_user_registration_success_and_returns_jwt() {
    let user_data = create_registration_data();

    let mut repository = MockPgRepositoryContract::new();
    let mut service = MockPgServiceContract::new();

    repository
        .expect_exists()
        .return_const(false);

    service
        .expect_create_user()
        .return_once(move |r| Ok(testable(&r.email, None, None, None, None)));

    let obj = Registration {
        repository,
        service,
    };

    let (authenticated_user, token) = obj.register(user_data).await.unwrap();
    assert_eq!(&authenticated_user.email, "testmail@gmail.com");
    assert!(!&token.is_empty());
}

fn create_registration_data() -> RegistrationUserData {
    RegistrationUserData { 
        email: Some("testmail@gmail.com".to_string()), 
        first_name: Some("John".to_string()), 
        last_name: Some("Doe".to_string()), 
        nick_name: Some("Johnny".to_string()),
        password: Some("test".to_string()),
        avatar: Some("test/image".to_string())
    }
}