use diesel::{Insertable, Queryable, RunQueryDsl};
use error::Error;
use infrastructure::{schema::watch_images, db::DbConnection};
use serde::{Serialize, Deserialize};

/// Struct for holding post data fron database
#[derive(Insertable, Queryable, Serialize, Deserialize, PartialEq, Debug, Clone)]
#[diesel(table_name = watch_images)]
#[diesel(treat_none_as_null = true)]
#[serde(rename_all = "camelCase")]
pub struct WatchImage {
    pub id: String,
    pub watch_id: String,
    pub data: Vec<u8>,
}

impl WatchImage {
    /// Method for creating watch image
    pub fn create(data: CreateNewWatchImageData, mut connection: DbConnection) -> Result<WatchImage, Error> {
        diesel::insert_into(watch_images::table)
            .values(data)
            .get_result::<WatchImage>(&mut connection)
            .map_err(Error::from)
    }
}

impl From<WatchImage> for CreateNewWatchImageData {
    fn from(value: WatchImage) -> Self {
        CreateNewWatchImageData { 
            watch_id: value.watch_id,
            data: value.data,
        }
    }
}

/// Struct for creating post_like from PostLikeData
#[derive(Serialize, Deserialize, Insertable, PartialEq, Eq, Debug, Clone)]
#[diesel(table_name = watch_images)]
#[serde(rename_all = "camelCase")]
pub struct CreateNewWatchImageData {
    pub watch_id: String,
    pub data: Vec<u8>,
}