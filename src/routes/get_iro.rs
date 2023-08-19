use crate::database::iro::Entity as Iro;
use axum::{
    extract::{Path},
    http::StatusCode,
    Extension, Json,
};
use sea_orm::{DatabaseConnection, EntityTrait};
use serde::{Serialize};
use sea_orm::entity::prelude::*;

#[derive(Serialize)]
pub struct ResponseIro {
    iro_id: i32,
    iro_name: String,
    iro_address_1: Option<String>,
    iro_address_2: Option<String>,
    iro_zip: Option<String>,
    iro_contact_f_name: Option<String>,
    iro_contact_l_name: Option<String>,
    iro_email: Option<String>,
    iro_license_expiration: Option<Date>,
}

pub async fn get_iro(
    Path(iro_id): Path<i32>,
    Extension(database): Extension<DatabaseConnection>
) -> Result<Json<ResponseIro>, StatusCode> {
    let iro = Iro::find_by_id(iro_id)
    .one(&database)
    .await
    .unwrap();

    if let Some(iro) = iro {
        Ok(Json(ResponseIro {
            iro_id: iro.iro_id,
            iro_name: iro.iro_name,
            iro_address_1: iro.iro_address_1,
            iro_address_2: iro.iro_address_2,
            iro_zip: iro.iro_zip,
            iro_contact_f_name: iro.iro_contact_f_name,
            iro_contact_l_name: iro.iro_contact_l_name,
            iro_email: iro.iro_email,
            iro_license_expiration: iro.iro_license_expiration,
        }))
    } else {
        Err(StatusCode::NOT_FOUND)
    }
}

pub async fn get_all_iros(Extension(database): Extension<DatabaseConnection>) -> Result<Json<Vec<ResponseIro>>, StatusCode> {
    let iros = Iro::find()
        // .filter(priority_filter)
        // .filter(tasks::Column::DeletedAt.is_null())
        .all(&database)
        .await
        .map_err(|_error| StatusCode::INTERNAL_SERVER_ERROR)?
        .into_iter()
        .map(|db_iro| ResponseIro {
            iro_id: db_iro.iro_id,
            iro_name: db_iro.iro_name,
            iro_address_1: db_iro.iro_address_1,
            iro_address_2: db_iro.iro_address_2,
            iro_zip: db_iro.iro_zip,
            iro_contact_f_name: db_iro.iro_contact_f_name,
            iro_contact_l_name: db_iro.iro_contact_l_name,
            iro_email: db_iro.iro_email,
            iro_license_expiration: db_iro.iro_license_expiration,
        })
        .collect();

    Ok(Json(iros))

}