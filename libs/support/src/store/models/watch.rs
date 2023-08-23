use chrono::NaiveDateTime;
use diesel::{Insertable, Queryable, RunQueryDsl, QueryDsl, ExpressionMethods, AsChangeset, prelude::Identifiable, Selectable};
use infrastructure::{
    db::DbConnection,
    schema::{watches, watches::dsl::watches as edit_watch}
};
use error::Error;
use serde::{Serialize, Deserialize};

#[derive(Insertable, Queryable, Serialize, Deserialize, Identifiable, Selectable, PartialEq, Eq, Debug, Clone)]
#[diesel(table_name = watches)]
#[diesel(treat_none_as_null = true)]
#[serde(rename_all = "camelCase")]
pub struct Watch {
    pub id: String,
    pub brand: String,
    pub model: String,
    pub diameter: i32,
    pub lug_width: i32,
    pub case_material: String,
    pub mechanism_model: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl Watch {
    /// Method for creating watch
    pub fn create(data: CreateNewWatchData, mut connection: DbConnection) -> Result<Watch, Error> {
        diesel::insert_into(watches::table)
            .values(data)
            .get_result::<Watch>(&mut connection)
            .map_err(Error::from)
    }

    /// Helper method to find watch by id
    pub fn get_by_id(id: &str, connection: &mut DbConnection) -> Result<Watch, Error> {
        watches::table
            .filter(watches::id.eq(id))
            .get_result::<Watch>(connection)
            .map_err(Error::from)
    }

    /// Helper method to edit watch by id
    pub fn edit(id: &str, data: CreateNewWatchData, connection: &mut DbConnection) -> Result<Watch, Error> {
        diesel::update(edit_watch)
            .filter(watches::id.eq(id))
            .set(data)
            .get_result::<Watch>(connection)
            .map_err(Error::from)
    }
}

impl From<Watch> for CreateNewWatchData {
    fn from(value: Watch) -> Self {
        CreateNewWatchData { 
            brand: value.brand,
            model: value.model,
            diameter: value.diameter,
            lug_width: value.lug_width,
            case_material: value.case_material,
            mechanism_model: value.mechanism_model,
        }
    }
}

/// Struct for creating Watch from post data
#[derive(Serialize, Deserialize, Insertable, AsChangeset, PartialEq, Eq, Debug, Clone)]
#[diesel(table_name = watches)]
#[serde(rename_all = "camelCase")]
pub struct CreateNewWatchData {
    pub brand: String,
    pub model: String,
    pub diameter: i32,
    pub lug_width: i32,
    pub case_material: String,
    pub mechanism_model: String,
}