use crate::database::provider::{self, Entity as Provider};
use axum::{
    extract::{Path, Extension, Json},
    http::StatusCode,
};
use sea_orm::{DatabaseConnection, Set, ColumnTrait, QueryFilter, EntityTrait, prelude::DateTimeWithTimeZone};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct RequestProvider {
    pub provider_id: Option<i32>,
    pub provider_name: String,
    pub provider_address_1: Option<String>,
    pub provider_address_2: Option<String>,
    pub provider_zip: Option<String>,
    pub provider_contact_f_name: Option<String>,
    pub provider_contact_l_name: Option<String>,
    pub provider_phone: Option<String>,
    pub deleted_at: Option<DateTimeWithTimeZone>,
    pub created_by: Option<i32>,
}

pub async fn atomic_update_provider(
    Path(provider_id): Path<i32>,
    Extension(database): Extension<DatabaseConnection>,
    Json(request_provider): Json<RequestProvider>
) -> Result<(), StatusCode> {
    let update_provider = provider::ActiveModel {
        // Not allowing them to update ID
        provider_id: Set(provider_id),
        provider_name: Set(request_provider.provider_name),
        provider_address_1: Set(request_provider.provider_address_1),
        provider_address_2: Set(request_provider.provider_address_2),
        provider_zip: Set(request_provider.provider_zip),
        provider_contact_f_name: Set(request_provider.provider_contact_f_name),
        provider_contact_l_name: Set(request_provider.provider_contact_l_name),
        provider_phone: Set(request_provider.provider_phone),
        deleted_at: Set(request_provider.deleted_at),
        created_by: Set(request_provider.created_by),
        
    };

    Provider::update(update_provider)
        .filter(provider::Column::ProviderId.eq(provider_id))
        .exec(&database)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(())
}