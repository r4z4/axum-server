use crate::database::patient::{self, Entity as Patient};
use axum::{
    extract::{Path, Extension, Query},
    http::StatusCode,
};
use sea_orm::{DatabaseConnection, Set, EntityTrait, IntoActiveModel};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct QueryParams {
    soft: bool,
}

pub async fn delete_patient(
    Path(patient_id): Path<i32>,
    Extension(database): Extension<DatabaseConnection>,
    Query(query_params): Query<QueryParams>
) -> Result<(), StatusCode> {
    let mut patient = if let Some(patient) = Patient::find_by_id(patient_id)
        .one(&database)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)? 
        {
            patient.into_active_model()
        } else {
            return Err(StatusCode::NOT_FOUND);
        };

    if query_params.soft {
        let now = chrono::Utc::now();
        patient.deleted_at = Set(Some(now.into()));
        Patient::update(patient)
            .exec(&database)
            .await
            .map_err(|_error| StatusCode::INTERNAL_SERVER_ERROR)?;
    } else {
        Patient::delete(patient)
            .exec(&database)
            .await
            .map_err(|_error| StatusCode::INTERNAL_SERVER_ERROR)?;
    }

    // Patient::delete_many()
    //     .filter(patient::Column:PatientId.eq(patient_id))
    //     .exec(&database)
    //     .await
    //     .map_err(|_error| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(())
}