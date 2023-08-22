use crate::database::insurer;
use axum::{
    extract::{Extension, Json},
    http::StatusCode,
};
use sea_orm::{DatabaseConnection, Set, prelude::Date, ActiveModelTrait};
use serde::{Serialize, Deserialize};

#[derive(Deserialize)]
pub struct RequestInsurer {
    insurer_name: String,
    insurer_phone: Option<String>,
    insurer_zip: Option<String>,
}

#[axum_macros::debug_handler]
pub async fn create_insurer(
    Extension(database): Extension<DatabaseConnection>,
    Json(request_insurer): Json<RequestInsurer> 
) {
    let new_insurer = insurer::ActiveModel{ 
        insurer_name: Set("BCBS".to_owned()),
        insurer_phone: Set(Some("402-111-1111".to_owned())),
        insurer_email: Set(Some("main@bcbs.com".to_owned())),
        insurer_address_1: Set(Some("7777 Aksarben Ave.".to_owned())),
        insurer_address_2: Set(Some("Suite 730".to_owned())),
        insurer_zip: Set(Some("68124".to_owned())),
        insurer_contact_f_name: Set(Some("Barry".to_owned())),
        insurer_contact_l_name: Set(Some("Blue".to_owned())),
        ..Default::default()
     };

     let result = new_insurer.save(&database).await.unwrap();

     dbg!(result);
}