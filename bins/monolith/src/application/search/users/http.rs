use actix_web::{HttpRequest, web, HttpResponse};
use error::Error;
use validr::Validation;
use super::{contract::GetSearchedUsers, data::SearchAttributes};

pub async fn handle_get_searched_users<T: GetSearchedUsers>(
    _req: HttpRequest,
    attributes: web::Query<SearchAttributes>,
    service: web::Data<T>
) -> Result<HttpResponse, Error> {
    let attributes = attributes.into_inner().validate()?;

    let response = 
        service
            .search_users(attributes)
            .await?;

    Ok(HttpResponse::Ok()
        .json(response)
    )
}