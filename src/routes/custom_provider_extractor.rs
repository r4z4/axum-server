use axum::{
    async_trait, 
    extract::{FromRequest}, 
    http::{Request, StatusCode}, 
    BoxError, 
    body::HttpBody,
    Json,
RequestExt};
use serde::Deserialize;
use validator::Validate;

#[derive(Deserialize, Debug, Validate)]
pub struct RequestProvider {
    pub provider_id: Option<i32>,
    #[validate(length(min = 2, message = "Provider name must have at least 2 characters"))]
    pub provider_name: String,
    #[validate(length(min = 4, message = "Provider address must have at least 4 characters"))]
    pub provider_address_1: Option<String>,
    pub provider_address_2: Option<String>,
    pub provider_zip: Option<String>,
    pub provider_contact_f_name: Option<String>,
    pub provider_contact_l_name: Option<String>,
    pub provider_phone: Option<String>,
    // #[validate(email(message = "must be a valid email"))]
}

#[async_trait]
impl<S, B> FromRequest<S, B> for RequestProvider
where
    // these bounds are required by `async_trait`
    B: HttpBody + Send + 'static,
    B::Data: Send,
    B::Error: Into<BoxError>,
    S: Send + Sync,
{
    type Rejection = (StatusCode, String);

    async fn from_request(req: Request<B>, state: &S) -> Result<Self, Self::Rejection> {
        let Json(provider) = req
            .extract::<Json<RequestProvider>, _>()
            .await
            .map_err(|error| {
                (StatusCode::BAD_REQUEST, format!("{}", error))
            })?;
        
            if let Err(errors) = provider.validate() {
                return Err((StatusCode::BAD_REQUEST, format!("{}", errors)));
            }

            Ok(provider)
    }
}

pub async fn custom_provider_extractor(provider: RequestProvider) {
    dbg!(provider);
}