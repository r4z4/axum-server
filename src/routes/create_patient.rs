use crate::database::patient;
use axum::{
    extract::{Extension, Json},
};
use sea_orm::{DatabaseConnection, Set, prelude::Date, ActiveModelTrait};
use serde::{Deserialize};

#[derive(Deserialize)]
pub struct RequestPatient {
    patient_f_name: String,
    patient_l_name: String,
    patient_email: Option<String>,
    patient_zip: Option<String>,
    patient_address_1: Option<String>,
    patient_address_2: Option<String>,
    patient_dob: Option<String>,
}

#[axum_macros::debug_handler]
pub async fn create_patient(
    Extension(database): Extension<DatabaseConnection>,
    Json(request_patient): Json<RequestPatient> 
) {
    let default_dob = "1970-01-01";
    let input_year: i32 = request_patient.patient_dob.as_ref().unwrap_or(&default_dob.to_owned()).split("-").nth(0).unwrap().parse::<i32>().unwrap();
    let input_month: u32  = request_patient.patient_dob.as_ref().unwrap_or(&default_dob.to_owned()).split("-").nth(1).unwrap().parse::<u32>().unwrap();
    let input_day: u32  = request_patient.patient_dob.as_ref().unwrap_or(&default_dob.to_owned()).split("-").nth(2).unwrap().parse::<u32>().unwrap();
    let new_patient = patient::ActiveModel{ 
        patient_f_name: Set(request_patient.patient_f_name),
        patient_l_name: Set(request_patient.patient_l_name),
        patient_email: Set(request_patient.patient_email),
        patient_address_1: Set(request_patient.patient_address_1),
        patient_address_2: Set(request_patient.patient_address_2),
        patient_zip: Set(request_patient.patient_zip),
        patient_dob: Set(Some(Date::from_ymd_opt(input_year, input_month, input_day).unwrap())),
        ..Default::default()
     };

     let result = new_patient.save(&database).await.unwrap();

     dbg!(result);
}