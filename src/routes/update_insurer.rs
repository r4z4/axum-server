use crate::database::insurer::{self, Entity as Insurer};
use axum::{
    extract::{Path, Extension, Json},
    http::StatusCode,
};
use sea_orm::{DatabaseConnection, Set, ColumnTrait, QueryFilter, EntityTrait, prelude::DateTimeWithTimeZone};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct RequestInsurer {
    pub insurer_id: Option<i32>,
    pub insurer_name: String,
    pub insurer_address_1: Option<String>,
    pub insurer_address_2: Option<String>,
    pub insurer_zip: Option<String>,
    pub insurer_email: Option<String>,
    pub insurer_contact_f_name: Option<String>,
    pub insurer_contact_l_name: Option<String>,
    pub insurer_phone: Option<String>,
    pub deleted_at: Option<DateTimeWithTimeZone>,
}

pub async fn atomic_update_insurer(
    Path(insurer_id): Path<i32>,
    Extension(database): Extension<DatabaseConnection>,
    Json(request_insurer): Json<RequestInsurer>
) -> Result<(), StatusCode> {
    let update_insurer = insurer::ActiveModel {
        insurer_id: Set(insurer_id),
        insurer_name: Set(request_insurer.insurer_name),
        insurer_address_1: Set(request_insurer.insurer_address_1),
        insurer_address_2: Set(request_insurer.insurer_address_2),
        insurer_zip: Set(request_insurer.insurer_zip),
        insurer_email: Set(request_insurer.insurer_email),
        insurer_contact_f_name: Set(request_insurer.insurer_contact_f_name),
        insurer_contact_l_name: Set(request_insurer.insurer_contact_l_name),
        insurer_phone: Set(request_insurer.insurer_phone),
        deleted_at: Set(request_insurer.deleted_at),
    };

    Insurer::update(update_insurer)
        .filter(insurer::Column::InsurerId.eq(insurer_id))
        .exec(&database)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(())
}