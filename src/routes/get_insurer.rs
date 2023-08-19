use crate::database::insurer::Entity as Insurer;
use axum::{
    extract::{Path},
    http::StatusCode,
    Extension, Json,
};
use sea_orm::{DatabaseConnection, EntityTrait};
use serde::{Serialize};

#[derive(Serialize)]
pub struct ResponseInsurer {
    insurer_id: i32,
    insurer_name: String,
    insurer_phone: Option<String>,
    insurer_email: Option<String>,
    insurer_address_1: Option<String>,
    insurer_address_2: Option<String>,
    insurer_zip: Option<String>,
    insurer_contact_f_name: Option<String>,
    insurer_contact_l_name: Option<String>,
}

pub async fn get_insurer(
    Path(insurer_id): Path<i32>,
    Extension(database): Extension<DatabaseConnection>
) -> Result<Json<ResponseInsurer>, StatusCode> {
    let insurer = Insurer::find_by_id(insurer_id)
    .one(&database)
    .await
    .unwrap();

    if let Some(insurer) = insurer {
        Ok(Json(ResponseInsurer {
            insurer_id: insurer.insurer_id,
            insurer_name: insurer.insurer_name,
            insurer_phone: insurer.insurer_phone,
            insurer_email: insurer.insurer_email,
            insurer_address_1: insurer.insurer_address_1,
            insurer_address_2: insurer.insurer_address_2,
            insurer_zip: insurer.insurer_zip,
            insurer_contact_f_name: insurer.insurer_contact_f_name,
            insurer_contact_l_name: insurer.insurer_contact_l_name,
        }))
    } else {
        Err(StatusCode::NOT_FOUND)
    }
}

pub async fn get_all_insurers(Extension(database): Extension<DatabaseConnection>) -> Result<Json<Vec<ResponseInsurer>>, StatusCode> {
    let insurers = Insurer::find()
        // .filter(priority_filter)
        // .filter(tasks::Column::DeletedAt.is_null())
        .all(&database)
        .await
        .map_err(|_error| StatusCode::INTERNAL_SERVER_ERROR)?
        .into_iter()
        .map(|db_insurer| ResponseInsurer {
            insurer_id: db_insurer.insurer_id,
            insurer_name: db_insurer.insurer_name,
            insurer_phone: db_insurer.insurer_phone,
            insurer_email: db_insurer.insurer_email,
            insurer_address_1: db_insurer.insurer_address_1,
            insurer_address_2: db_insurer.insurer_address_2,
            insurer_zip: db_insurer.insurer_zip,
            insurer_contact_f_name: db_insurer.insurer_contact_f_name,
            insurer_contact_l_name: db_insurer.insurer_contact_l_name,
        })
        .collect();

    Ok(Json(insurers))

}