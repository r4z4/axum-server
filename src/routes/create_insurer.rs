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
    insurer_email: Option<String>,
    insurer_address_1: Option<String>,
    insurer_address_2: Option<String>,
    insurer_contact_f_name: Option<String>,
    insurer_contact_l_name: Option<String>,
}

#[axum_macros::debug_handler]
pub async fn create_insurer(
    Extension(database): Extension<DatabaseConnection>,
    Json(request_insurer): Json<RequestInsurer> 
) {
    let new_insurer = insurer::ActiveModel{ 
        insurer_name: Set(request_insurer.insurer_name),
        insurer_phone: Set(request_insurer.insurer_phone),
        insurer_email: Set(request_insurer.insurer_email),
        insurer_address_1: Set(request_insurer.insurer_address_1),
        insurer_address_2: Set(request_insurer.insurer_address_2),
        insurer_zip: Set(request_insurer.insurer_zip),
        insurer_contact_f_name: Set(request_insurer.insurer_contact_f_name),
        insurer_contact_l_name: Set(request_insurer.insurer_contact_l_name),
        ..Default::default()
     };

     let result = new_insurer.save(&database).await.unwrap();

     dbg!(result);
}