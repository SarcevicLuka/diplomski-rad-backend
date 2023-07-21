use std::vec;
use validr::*;
use serde::{Serialize, Deserialize};
use support::store::models::user::CreateNewUserData;

/// Struct for holding basic registration data
#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RegistrationUserData {
    pub email: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub nick_name: Option<String>,
    pub avatar: Option<String>,
    pub password: Option<String>,
}

impl Validation for RegistrationUserData {
    fn rules(&self) -> Vec<Rule<Self>> {
        vec![
            rule_required!(email),
            rule_email!(email),
            rule_required!(password),
            Rule::new("password", |obj: &Self, error| {
                if let Some(v) = &obj.password {
                    if support::helpers::validations::password_strength(v) {
                        error.add("Password too weak");
                    }
                }
            }),
            rule_required!(first_name),
            rule_length_max!(first_name, 50),
            rule_length_min!(first_name, 2),
            rule_required!(last_name),
            rule_length_max!(last_name, 50),
            rule_length_min!(last_name, 2),
            rule_required!(nick_name),
            rule_length_max!(first_name, 50),
            rule_length_min!(first_name, 2)
        ]
    }
    fn modifiers(&self) -> Vec<Modifier<Self>> {
        vec![
            modifier_lowercase!(email),
            modifier_trim!(first_name),
            modifier_trim!(last_name)
        ]
    }
}

impl RegistrationUserData {
    /// Return struct that can be inserted into the database
    pub fn insertable(self) -> CreateNewUserData {
        let email = match self.email {
            Some(v) => v,
            None => "".to_string(),
        };

        let first_name = match self.first_name {
            Some(v) => v,
            None => "".to_string(),
        };

        let last_name = match self.last_name {
            Some(v) => v,
            None => "".to_string(),
        };

        let nick_name = match self.nick_name {
            Some(v) => v,
            None => "".to_string(),
        };

        let password = match self.password {
            Some(v) => v,
            None => "".to_string(),
        };

        let avatar = match self.avatar {
            Some(v) => v,
            None => "".to_string()
        };

        CreateNewUserData {
            email,
            first_name,
            last_name,
            nick_name,
            password,
            avatar
        }
    }
}