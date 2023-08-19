use crate::database::patient::Entity as Patient;
use axum::{
    extract::{Path},
    http::StatusCode,
    Extension, Json,
};
use sea_orm::{DatabaseConnection, EntityTrait};
use serde::{Serialize};

#[derive(Serialize)]
pub struct ResponsePatient {
    patient_id: i32,
    patient_f_name: String,
    patient_l_name: String,
    patient_email: Option<String>,
}

pub async fn get_patient(
    Path(patient_id): Path<i32>,
    Extension(database): Extension<DatabaseConnection>
) -> Result<Json<ResponsePatient>, StatusCode> {
    let patient = Patient::find_by_id(patient_id)
    .one(&database)
    .await
    .unwrap();

    if let Some(patient) = patient {
        Ok(Json(ResponsePatient {
            patient_id: patient.patient_id,
            patient_f_name: patient.patient_f_name,
            patient_l_name: patient.patient_l_name,
            patient_email: patient.patient_email,
        }))
    } else {
        Err(StatusCode::NOT_FOUND)
    }
}

pub async fn get_all_patients(Extension(database): Extension<DatabaseConnection>) -> Result<Json<Vec<ResponsePatient>>, StatusCode> {
    let patients = Patient::find()
        // .filter(priority_filter)
        // .filter(tasks::Column::DeletedAt.is_null())
        .all(&database)
        .await
        .map_err(|_error| StatusCode::INTERNAL_SERVER_ERROR)?
        .into_iter()
        .map(|db_patient| ResponsePatient {
            patient_id: db_patient.patient_id,
            patient_f_name: db_patient.patient_f_name,
            patient_l_name: db_patient.patient_l_name,
            patient_email: db_patient.patient_email,
        })
        .collect();

    Ok(Json(patients))

}