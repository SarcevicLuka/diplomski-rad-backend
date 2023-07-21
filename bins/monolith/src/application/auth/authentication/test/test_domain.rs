use error::Error;
use support::store::models::user::testable;
use crate::application::auth::authentication::{
    domain::Login,
    data::LoginUserData, 
    contract::{
        MockPgRepositoryContract, 
        LoginContract
    }, 
};

#[actix_web::main]
#[test]
async fn test_user_login_success() {
    let user_data = LoginUserData {
        email: Some("testmail@gmail.com".to_string()),
        password: Some("test".to_string()),
    };

    let mut repository = MockPgRepositoryContract::new();
    repository
        .expect_get_user_by_email()
        .return_once(move |email| Ok(testable(email, None, None, None, None)));

    let obj = Login {
        repository,
    };

    let (user, _token) = obj.login(user_data).await.unwrap();
    assert_eq!(user.email, "testmail@gmail.com");
}

#[actix_web::main]
#[test]
async fn test_user_login_success_and_returns_jwt() {
    let user_data = LoginUserData {
        email: Some("testmail@gmail.com".to_string()),
        password: Some("test".to_string()),
    };

    let mut repository = MockPgRepositoryContract::new();
    repository
        .expect_get_user_by_email()
        .return_once(move |email| Ok(testable(email, None, None, None, None)));

    let obj = Login {
        repository,
    };

    let (user, token) = obj.login(user_data).await.unwrap();
    assert_eq!(user.email, "testmail@gmail.com");
    assert!(!&token.is_empty());
}

#[actix_web::main]
#[test]
async fn test_user_login_fail_no_user_with_email() {
    let user_data = LoginUserData {
        email: Some("testinvalidemail@gmail.com".to_string()),
        password: Some("test".to_string()),
    };

    let mut repository = MockPgRepositoryContract::new();
    repository
        .expect_get_user_by_email()
        .return_once(move |_| Err(Error::Forbidden("user with given email does not exist".to_string())));

    let obj = Login {
        repository,
    };

    let response = obj.login(user_data).await;
    match response {
        Ok(_) => panic!("Should have been error"),
        Err(e) => match e {
            Error::Forbidden(cause) => {
                assert_eq!(cause, "user with given email does not exist".to_string())
            }
            _ => panic!("Wrong error recieved"),
        }
    };
}

#[actix_web::main]
#[test]
async fn test_user_login_fail_password_incorrect() {
    let user_data = LoginUserData {
        email: Some("testmail@gmail.com".to_string()),
        password: Some("incorrectPassword".to_string()),
    };

    let mut repository = MockPgRepositoryContract::new();
    repository
        .expect_get_user_by_email()
        .return_once(move |email| Ok(testable(email, None, None, Some("correctPassword"), None)));

    let obj = Login {
        repository,
    };

    let response= obj.login(user_data).await;
    match response {
        Ok(_) => panic!("Should have been error"),
        Err(e) => match e {
            Error::Forbidden(cause) => {
                assert_eq!(cause, "Invalid credentials".to_string())
            }
            _ => panic!("Wrong error recieved"),
        }
    };
}