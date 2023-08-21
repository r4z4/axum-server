use crate::database::patient::{self, Entity as Patient};
use axum::{
    extract::{Path, Extension, Json},
    http::StatusCode,
};
use sea_orm::{DatabaseConnection, Set, ColumnTrait, QueryFilter, EntityTrait, prelude::Date, prelude::DateTimeWithTimeZone};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct RequestPatient {
    pub patient_id: Option<i32>,
    pub patient_email: Option<String>,
    pub patient_address_1: Option<String>,
    pub patient_address_2: Option<String>,
    pub patient_zip: Option<String>,
    pub patient_f_name: String,
    pub patient_l_name: String,
    pub patient_dob: Option<Date>,
    pub deleted_at: Option<DateTimeWithTimeZone>,
}

pub async fn atomic_update_patient(
    Path(patient_id): Path<i32>,
    Extension(database): Extension<DatabaseConnection>,
    Json(request_patient): Json<RequestPatient>
) -> Result<(), StatusCode> {
    let update_patient = patient::ActiveModel {
        patient_id: Set(patient_id),
        patient_email: Set(request_patient.patient_email),
        patient_address_1: Set(request_patient.patient_address_1),
        patient_address_2: Set(request_patient.patient_address_2),
        patient_zip: Set(request_patient.patient_zip),
        patient_f_name: Set(request_patient.patient_f_name),
        patient_l_name: Set(request_patient.patient_l_name),
        patient_dob: Set(request_patient.patient_dob),
        deleted_at: Set(request_patient.deleted_at),
    };

    Patient::update(update_patient)
        .filter(patient::Column::PatientId.eq(patient_id))
        .exec(&database)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(())
}