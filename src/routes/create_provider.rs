use crate::database::provider;
use axum::{
    extract::{Extension, Json},
    http::StatusCode,
};
use sea_orm::{DatabaseConnection, Set};
use sea_orm::{ActiveModelTrait};
use serde::{Serialize, Deserialize};

#[derive(Deserialize)]
pub struct RequestProvider {
    provider_name: String,
    provider_phone: Option<String>,
    provider_zip: Option<String>,
}

#[axum_macros::debug_handler]
pub async fn create_provider(
    Extension(database): Extension<DatabaseConnection>,
    Json(request_provider): Json<RequestProvider> 
) {
    let new_provider = provider::ActiveModel{ 
        provider_name: Set(request_provider.provider_name),
        provider_phone: Set(request_provider.provider_phone),
        provider_zip: Set(request_provider.provider_zip),
        provider_address_1: Set(Some("1234 Main St.".to_owned())),
        provider_address_2: Set(Some("Ste. 22".to_owned())),
        provider_contact_f_name: Set(Some("Paul".to_owned())),
        provider_contact_l_name: Set(Some("Provider".to_owned())),
        ..Default::default()
     };

     let result = new_provider.save(&database).await.unwrap();

     dbg!(result);
}