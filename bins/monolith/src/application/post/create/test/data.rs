// use crate::application::post::create::data::{UserPostData, WatchData};

// #[allow(dead_code)]
// pub fn generate_create_post_data(
//     watch_data: Option<WatchData>, 
//     review: Option<&str>,
//     score: Option<i32>
// ) -> UserPostData {
//     UserPostData { 
//         watch_data: watch_data.unwrap(), 
//         review: Some(review.unwrap().to_string()), 
//         score
//     }
// }

// #[allow(dead_code)]
// pub fn generate_create_watch_data() -> WatchData {
//     WatchData { 
//         brand: Some("brand".to_string()), 
//         model: Some("brand".to_string()),
//         diameter: Some(40), 
//         lug_width: Some(20), 
//         case_material: Some("case material".to_string()),
//         mechanism_model: Some("mechanism model".to_string()), 
//     }
// }