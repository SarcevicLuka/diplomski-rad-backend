use actix_web::{HttpRequest, web, HttpResponse, HttpMessage};
use error::Error;
use support::{
    store::models::user::DisplayUser, 
    helpers::http::part_from_path
};
use crate::application::post::comment::like::contract::LikeCommentContract;

pub async fn handle_remove_like_comment<T: LikeCommentContract>(
    req: HttpRequest,
    service: web::Data<T>,
) -> Result<HttpResponse, Error> {
    let Some(user) = req.extensions_mut().remove::<DisplayUser>() else {
        return Err(Error::Unauthorized("Not authorized".to_string()));
    };

    let comment_id = part_from_path::<String>(&req, "comment_id")?;

    service
        .remove_like_comment(&user.id, &comment_id)
        .await?;

    Ok(HttpResponse::Ok().finish())
}