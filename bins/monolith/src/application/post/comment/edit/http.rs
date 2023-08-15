use actix_web::{HttpRequest, web, HttpResponse, HttpMessage};
use error::Error;
use validr::Validation;
use support::{store::models::user::DisplayUser, helpers::http::part_from_path};
use super::{contract::EditCommentContract, data::UserEditCommentData};

pub async fn handle_edit_comment<T: EditCommentContract>(
    req: HttpRequest,
    service: web::Data<T>,
    data: web::Json<UserEditCommentData>
) -> Result<HttpResponse, Error> {
    let Some(_user) = req.extensions_mut().remove::<DisplayUser>() else {
        return Err(Error::Unauthorized("Not authorized".to_string()));
    };

    let data = data.into_inner().validate()?;
    let comment_id = part_from_path::<String>(&req, "comment_id")?;

    let response = 
        service
            .edit_comment(&comment_id, data)
            .await?;

    Ok(HttpResponse::Ok()
        .json(response)
    )
}