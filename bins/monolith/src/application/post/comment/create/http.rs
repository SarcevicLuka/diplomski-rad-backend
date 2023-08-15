use actix_web::{HttpRequest, web, HttpResponse, HttpMessage};
use error::Error;
use validr::Validation;
use support::{store::models::user::DisplayUser, helpers::http::part_from_path};
use super::{contract::CreateCommentContract, data::UserCommentData};

pub async fn handle_create_comment<T: CreateCommentContract>(
    req: HttpRequest,
    service: web::Data<T>,
    data: web::Json<UserCommentData>
) -> Result<HttpResponse, Error> {
    let Some(user) = req.extensions_mut().remove::<DisplayUser>() else {
        return Err(Error::Unauthorized("Not authorized".to_string()));
    };

    let data = data.into_inner().validate()?;
    let post_id = part_from_path::<String>(&req, "post_id")?;

    let response = 
        service
            .create_comment(&user.id, &post_id, data)
            .await?;

    Ok(HttpResponse::Created()
        .json(response)
    )
}