use crate::database::iro::{self, Entity as Iro};
use axum::{
    extract::{Path, Extension, Json},
    http::StatusCode,
};
use sea_orm::{DatabaseConnection, Set, ColumnTrait, QueryFilter, EntityTrait, prelude::Date};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct RequestIro {
    pub iro_id: Option<i32>,
    pub iro_name: String,
    pub iro_address_1: Option<String>,
    pub iro_address_2: Option<String>,
    pub iro_zip: Option<String>,
    pub iro_contact_f_name: Option<String>,
    pub iro_contact_l_name: Option<String>,
    pub iro_email: Option<String>,
    pub iro_license_expiration: Option<Date>,
}

pub async fn atomic_update_iro(
    Path(iro_id): Path<i32>,
    Extension(database): Extension<DatabaseConnection>,
    Json(request_iro): Json<RequestIro>
) -> Result<(), StatusCode> {
    let update_iro = iro::ActiveModel {
        iro_id: Set(iro_id),
        iro_name: Set(request_iro.iro_name),
        iro_address_1: Set(request_iro.iro_address_1),
        iro_address_2: Set(request_iro.iro_address_2),
        iro_zip: Set(request_iro.iro_zip),
        iro_contact_f_name: Set(request_iro.iro_contact_f_name),
        iro_contact_l_name: Set(request_iro.iro_contact_l_name),
        iro_email: Set(request_iro.iro_email),
        iro_license_expiration: Set(request_iro.iro_license_expiration),
    };

    Iro::update(update_iro)
        .filter(iro::Column::IroId.eq(iro_id))
        .exec(&database)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(())
}