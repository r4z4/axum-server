use crate::database::patient;
use axum::{
    extract::{Extension, Json},
    http::StatusCode,
};
use sea_orm::{DatabaseConnection, Set, prelude::Date, ActiveModelTrait};
use serde::{Serialize, Deserialize};

#[derive(Deserialize)]
pub struct RequestPatient {
    patient_f_name: String,
    patient_l_name: String,
    patient_email: Option<String>,
    patient_phone: Option<String>,
    patient_zip: Option<String>,
}

#[axum_macros::debug_handler]
pub async fn create_patient(
    Extension(database): Extension<DatabaseConnection>,
    Json(request_patient): Json<RequestPatient> 
) {
    let new_patient = patient::ActiveModel{ 
        patient_f_name: Set(request_patient.patient_f_name),
        patient_l_name: Set(request_patient.patient_l_name),
        patient_email: Set(request_patient.patient_email),
        patient_address_1: Set(Some("2323 South 78th St.".to_owned())),
        patient_address_2: Set(Some("Apt. 77".to_owned())),
        patient_zip: Set(request_patient.patient_zip),
        patient_dob: Set(Some(Date::from_ymd_opt(1955, 5, 17).unwrap())),
        ..Default::default()
     };

     let result = new_patient.save(&database).await.unwrap();

     dbg!(result);
}