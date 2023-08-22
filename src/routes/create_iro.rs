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
    iro_phone: Option<String>,
    iro_zip: Option<String>,
}

#[axum_macros::debug_handler]
pub async fn create_iro(
    Extension(database): Extension<DatabaseConnection>,
    Json(request_iro): Json<RequestIro> 
) {
    let new_iro = iro::ActiveModel{ 
        iro_name: Set("Maximus Federal Services".to_owned()),
        iro_email: Set(Some("main@maximus.com".to_owned())),
        iro_contact_f_name: Set(Some("Jim".to_owned())),
        iro_contact_l_name: Set(Some("Federale".to_owned())),
        iro_address_1: Set(Some("1022 Federal Ave.".to_owned())),
        iro_address_2: Set(Some("Suite 2".to_owned())),
        iro_zip: Set(Some("10110".to_owned())),
        iro_license_expiration: Set(Some(Date::from_ymd_opt(2019, 1, 1).unwrap())),
        ..Default::default()
     };

     let result = new_iro.save(&database).await.unwrap();

     dbg!(result);
}