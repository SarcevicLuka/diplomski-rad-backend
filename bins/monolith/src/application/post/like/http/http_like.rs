use actix_web::{HttpRequest, web, HttpResponse, HttpMessage};
use error::Error;
use support::{
    store::models::user::DisplayUser, 
    helpers::http::part_from_path
};
use crate::application::post::like::contract::LikePostContract;

pub async fn handle_like_post<T: LikePostContract>(
    req: HttpRequest,
    service: web::Data<T>,
) -> Result<HttpResponse, Error> {
    let Some(user) = req.extensions_mut().remove::<DisplayUser>() else {
        return Err(Error::Unauthorized("Not authorized".to_string()));
    };

    let post_id = part_from_path::<String>(&req, "post_id")?;

    let response = 
        service
            .like_post(&user.id, &post_id)
            .await?;

    Ok(HttpResponse::Ok()
        .json(response)
    )
}