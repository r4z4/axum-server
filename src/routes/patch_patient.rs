use crate::database::patient::{self, Entity as Patient};
use axum::{
    extract::{Path, Extension, Json},
    http::StatusCode,
};
use sea_orm::{DatabaseConnection, Set, ColumnTrait, QueryFilter, EntityTrait, IntoActiveModel, prelude::Date};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct RequestPatient {
    pub patient_id: Option<i32>,
    #[serde(
        default,                                    
        skip_serializing_if = "Option::is_none",    
        with = "::serde_with::rust::double_option",
    )]
    pub patient_dob: Option<Option<Date>>,
    #[serde(
        default,                                    
        skip_serializing_if = "Option::is_none",    
        with = "::serde_with::rust::double_option",
    )]
    pub patient_address_1: Option<Option<String>>,
    #[serde(
        default,                                    
        skip_serializing_if = "Option::is_none",    
        with = "::serde_with::rust::double_option",
    )]
    pub patient_address_2: Option<Option<String>>,
    #[serde(
        default,                                    
        skip_serializing_if = "Option::is_none",    
        with = "::serde_with::rust::double_option",
    )]
    pub patient_zip: Option<Option<String>>,
    pub patient_f_name: Option<String>,
    pub patient_l_name: Option<String>,
    #[serde(
        default,                                    
        skip_serializing_if = "Option::is_none",    
        with = "::serde_with::rust::double_option",
    )]
    pub patient_email: Option<Option<String>>,
}

pub async fn partial_update_patient(
    Path(patient_id): Path<i32>,
    Extension(database): Extension<DatabaseConnection>,
    Json(request_patient): Json<RequestPatient>
) -> Result<(), StatusCode> {
    let mut db_patient = if let Some(patient) = Patient::find_by_id(patient_id)
        .one(&database)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)? 
    {
        patient.into_active_model()
    } else {
        return Err(StatusCode::NOT_FOUND);
    };
    // Shouldn't enter this block if patient_name set to NULL or None
    if let Some(patient_dob) = request_patient.patient_dob {
        db_patient.patient_dob = Set(patient_dob)
    }
    if let Some(patient_address_1) = request_patient.patient_address_1 {
        db_patient.patient_address_1 = Set(patient_address_1)
    }
    if let Some(patient_address_2) = request_patient.patient_address_2 {
        db_patient.patient_address_2 = Set(patient_address_2)
    }
    if let Some(patient_zip) = request_patient.patient_zip {
        db_patient.patient_zip = Set(patient_zip)
    }
    if let Some(patient_f_name) = request_patient.patient_f_name {
        db_patient.patient_f_name = Set(patient_f_name)
    }
    if let Some(patient_l_name) = request_patient.patient_l_name {
        db_patient.patient_l_name = Set(patient_l_name)
    }
    if let Some(patient_email) = request_patient.patient_email {
        db_patient.patient_email = Set(patient_email)
    }

    Patient::update(db_patient)
        .filter(patient::Column::PatientId.eq(patient_id))
        .exec(&database)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(())
}