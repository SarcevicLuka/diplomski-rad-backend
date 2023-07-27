use actix_web::{HttpRequest, web, HttpResponse, HttpMessage};
use error::Error;
use validr::Validation;
use support::{
    store::models::user::DisplayUser, 
    helpers::http::part_from_path
};
use super::{
    contract::EditPostContract, 
    data::UserEditPostData
};

pub async fn handle_edit_post<T: EditPostContract>(
    req: HttpRequest,
    service: web::Data<T>,
    data: web::Json<UserEditPostData>
) -> Result<HttpResponse, Error> {
    let Some(_user) = req.extensions_mut().remove::<DisplayUser>() else {
        return Err(Error::Unauthorized("not authorized".to_string()));
    };

    let data = data.into_inner().validate()?;
    let post_id = part_from_path::<String>(&req, "post_id")?;

    let response = 
        service
            .edit_post(&post_id, data)
            .await?;

    Ok(HttpResponse::Ok()
        .json(response)
    )
}