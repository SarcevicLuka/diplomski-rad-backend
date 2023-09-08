use actix_web::{HttpRequest, web, HttpResponse};
use error::Error;
use validr::Validation;
use super::{
    contract::GetSearchedPosts, 
    super::users::data::SearchAttributes
};

pub async fn handle_get_searched_posts<T: GetSearchedPosts>(
    _req: HttpRequest,
    attributes: web::Query<SearchAttributes>,
    service: web::Data<T>
) -> Result<HttpResponse, Error> {
    let attributes = attributes.into_inner().validate()?;

    let response = 
        service
            .search_posts(attributes)
            .await?;

    Ok(HttpResponse::Ok()
        .json(response)
    )
}