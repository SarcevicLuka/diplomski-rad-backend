use std::vec;
use support::store::models::watch::CreateNewWatchData;
use validr::*;
use serde::{Serialize, Deserialize};

/// Struct for holding edited post data
#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UserEditPostData {
    pub watch_data: UserEditWatchData,
    //pub images: Vec<ImageData>,
    pub review: Option<String>
}

impl Validation for UserEditPostData {
    fn rules(&self) -> Vec<Rule<Self>> {
        vec![
            rule_required!(review),
            rule_length_max!(review, 200)
        ]
    }
    fn modifiers(&self) -> Vec<Modifier<Self>> {
        vec![]
    }
}

/// Struct for holding edited watch data
#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UserEditWatchData {
    pub brand: Option<String>,
    pub model: Option<String>,
    pub diameter: Option<i32>,
    pub lug_width: Option<i32>,
    pub case_material: Option<String>,
    pub mechanism_model: Option<String>,
}

impl Validation for UserEditWatchData {
    fn rules(&self) -> Vec<Rule<Self>> {
        vec![
            rule_required!(brand),
            rule_in!(brand, vec![
                "Seiko",
                "Rolex"
            ]),
            rule_required!(model),
            rule_required!(diameter),
            rule_range!(diameter, Some(1), Some(50)),
            rule_required!(lug_width),
            rule_range!(lug_width, Some(1), Some(30)),
            rule_required!(case_material),
            rule_in!(case_material, vec![
                "titanium",
                "aluminium",
                "plastic",
                "iron"
            ]),
            rule_required!(mechanism_model)
        ]
    }
    fn modifiers(&self) -> Vec<Modifier<Self>> {
        vec![]
    }
}

impl UserEditWatchData {
    /// Return struct that can be inserted into the database
    pub fn insertable(self) -> CreateNewWatchData {
        let brand = match self.brand {
            Some(v) => v,
            None => "".to_string(),
        };

        let model = match self.model {
            Some(v) => v,
            None => "".to_string(),
        };

        let diameter = self.diameter.unwrap_or(0);

        let lug_width = self.diameter.unwrap_or(0);

        let case_material = match self.case_material {
            Some(v) => v,
            None => "".to_string(),
        };

        let mechanism_model = match self.mechanism_model {
            Some(v) => v,
            None => "".to_string(),
        };

        CreateNewWatchData {
            brand,
            model,
            diameter,
            lug_width,
            case_material,
            mechanism_model
        }
    }
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ImageData {
    pub data: Option<String>
}

impl Validation for ImageData {
    fn rules(&self) -> Vec<Rule<Self>> {
        vec![
            rule_required!(data)
        ]
    }
    fn modifiers(&self) -> Vec<Modifier<Self>> {
        vec![]
    }
}