use serde::{Serialize, Deserialize};
use validr::*;

/// Struct for holding login data
#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct LoginUserData {
    pub email: Option<String>,
    pub password: Option<String>
}

impl Validation for LoginUserData {
    fn rules(&self) -> Vec<validr::Rule<Self>> {
        vec![
            rule_required!(email),
            rule_email!(email),
            rule_required!(password),
        ]
    }

    fn modifiers(&self) -> Vec<Modifier<Self>> {
        vec![
            modifier_lowercase!(email),
        ]
    }
}