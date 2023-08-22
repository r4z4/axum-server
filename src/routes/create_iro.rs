use crate::database::iro;
use axum::{
    extract::{Extension, Json},
    http::StatusCode,
};
use sea_orm::{DatabaseConnection, Set, prelude::Date, ActiveModelTrait};
use serde::{Serialize, Deserialize};

#[derive(Deserialize)]
pub struct RequestIro {
    iro_name: String,
    iro_email: Option<String>,
    iro_zip: Option<String>,
    iro_contact_f_name: Option<String>,
    iro_contact_l_name: Option<String>,
    iro_address_1: Option<String>,
    iro_address_2: Option<String>,
    iro_license_expiration: Option<String>,
}

#[axum_macros::debug_handler]
pub async fn create_iro(
    Extension(database): Extension<DatabaseConnection>,
    Json(request_iro): Json<RequestIro> 
) {
    let default_date = "2024-01-02";
    let input_year: i32 = request_iro.iro_license_expiration.as_ref().unwrap_or(&default_date.to_owned()).split("-").nth(0).unwrap().parse::<i32>().unwrap();
    let input_month: u32  = request_iro.iro_license_expiration.as_ref().unwrap_or(&default_date.to_owned()).split("-").nth(1).unwrap().parse::<u32>().unwrap();
    let input_day: u32  = request_iro.iro_license_expiration.as_ref().unwrap_or(&default_date.to_owned()).split("-").nth(2).unwrap().parse::<u32>().unwrap();
    let new_iro = iro::ActiveModel{ 
        iro_name: Set(request_iro.iro_name),
        iro_email: Set(request_iro.iro_email),
        iro_contact_f_name: Set(request_iro.iro_contact_f_name),
        iro_contact_l_name: Set(request_iro.iro_contact_l_name),
        iro_address_1: Set(request_iro.iro_address_1),
        iro_address_2: Set(request_iro.iro_address_2),
        iro_zip: Set(request_iro.iro_zip),
        iro_license_expiration: Set(Some(Date::from_ymd_opt(input_year, input_month, input_day).unwrap())),
        ..Default::default()
     };

     let result = new_iro.save(&database).await.unwrap();

     dbg!(result);
}