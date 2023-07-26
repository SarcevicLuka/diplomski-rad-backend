use std::vec;
use validr::*;
use serde::{Serialize, Deserialize};

/// Struct for holding post data that user wants to create
#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UserCommentData {
    pub text: Option<String>,
    pub score: Option<i32>
}

impl Validation for UserCommentData {
    fn rules(&self) -> Vec<Rule<Self>> {
        vec![
            rule_required!(text),
            rule_length_max!(text, 200),
            rule_required!(score),
            rule_range!(score, Some(0), Some(5))
        ]
    }
    fn modifiers(&self) -> Vec<Modifier<Self>> {
        vec![]
    }
}
