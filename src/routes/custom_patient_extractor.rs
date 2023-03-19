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
use sea_orm::entity::prelude::*;

#[derive(Deserialize, Debug, Validate)]
pub struct RequestPatient {
    pub patient_id: Option<i32>,
    pub patient_f_name: String,
    #[validate(length(min = 2, message = "Patient last name must have at least 2 characters"))]
    pub patient_l_name: String,
    #[validate(length(min = 4, message = "Patient address must have at least 4 characters"))]
    pub patient_address_1: Option<String>,
    pub patient_address_2: Option<String>,
    pub patient_zip: Option<String>,
    #[validate(email(message = "must be a valid email"))]
    pub patient_email: Option<String>,
    pub patient_dob: Option<Date>,
}

#[async_trait]
impl<S, B> FromRequest<S, B> for RequestPatient
where
    // these bounds are required by `async_trait`
    B: HttpBody + Send + 'static,
    B::Data: Send,
    B::Error: Into<BoxError>,
    S: Send + Sync,
{
    type Rejection = (StatusCode, String);

    async fn from_request(req: Request<B>, state: &S) -> Result<Self, Self::Rejection> {
        let Json(patient) = req
            .extract::<Json<RequestPatient>, _>()
            .await
            .map_err(|error| {
                (StatusCode::BAD_REQUEST, format!("{}", error))
            })?;
        
            if let Err(errors) = patient.validate() {
                return Err((StatusCode::BAD_REQUEST, format!("{}", errors)));
            }

            Ok(patient)
    }
}

pub async fn custom_patient_extractor(patient: RequestPatient) {
    dbg!(patient);
}