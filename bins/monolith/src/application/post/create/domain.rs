use async_trait::async_trait;
use error::Error;
use super::{
    contract::{CreatePostContract, PgRepositoryContract}, 
    data::UserPostData
};
use support::store::models::{
    post::{Post, CreateNewPostData},
    watch_images::CreateNewWatchImageData
};

pub struct CreatePost<
    A: PgRepositoryContract,
> {
    pub repository: A,
}

#[async_trait]
impl<A> CreatePostContract for CreatePost<A>
where
    A: PgRepositoryContract + Send + Sync,
{
    async fn create_post(
        &self,
        user_id: &str,
        post_data: UserPostData,
    ) -> Result<Post, Error> {
        let watch_id = self
            .repository
            .create_watch(post_data.watch_data.insertable())
            .await?;

        for data in post_data.images.into_iter() {
            match data.data {
                Some(image) => {
                    let watch_image_data = CreateNewWatchImageData {
                        watch_id: watch_id.clone(),
                        data: image.as_bytes().into()
                    };
                    self
                        .repository
                        .create_watch_image(watch_image_data)
                        .await?;
                }
                None => return Err(Error::Request("No images".to_string()))
            }
        }

        let post_data = CreateNewPostData {
            user_id: user_id.to_string(),
            watch_id,
            review: post_data.review.unwrap(),
            score: post_data.score.unwrap()
        };

        let post = self
            .repository
            .create_post(post_data)
            .await?;

        Ok(
            post
        )
    }
}