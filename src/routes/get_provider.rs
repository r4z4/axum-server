use crate::database::provider::{self, Entity as Provider};
use axum::{
    extract::{Path, Query, Json},
    http::StatusCode,
    Extension,
};
use sea_orm::{DatabaseConnection, EntityTrait, QueryFilter, ColumnTrait, Condition, prelude::DateTimeWithTimeZone};
use serde::{Serialize, Deserialize};

#[derive(Serialize)]
pub struct ResponseProvider {
    provider_id: i32,
    provider_name: String,
    provider_zip: Option<String>,
    provider_phone: Option<String>,
    provider_address_1: Option<String>,
    provider_address_2: Option<String>,
    provider_contact_f_name: Option<String>,
    provider_contact_l_name: Option<String>,
    deleted_at: Option<DateTimeWithTimeZone>,
    created_by: Option<i32>,
}

pub async fn get_provider(
    Path(provider_id): Path<i32>,
    Extension(database): Extension<DatabaseConnection>
) -> Result<Json<ResponseProvider>, StatusCode> {
    let provider = Provider::find_by_id(provider_id)
    .filter(provider::Column::DeletedAt.is_null())
    .one(&database)
    .await
    .unwrap();

    if let Some(provider) = provider {
        Ok(Json(ResponseProvider {
            provider_id: provider.provider_id,
            provider_name: provider.provider_name,
            provider_zip: provider.provider_zip,
            provider_phone: provider.provider_phone,
            provider_address_1: provider.provider_address_1,
            provider_address_2: provider.provider_address_2,
            provider_contact_f_name: provider.provider_contact_f_name,
            provider_contact_l_name: provider.provider_contact_l_name,
            deleted_at: provider.deleted_at,
            created_by: provider.created_by,
        }))
    } else {
        Err(StatusCode::NOT_FOUND)
    }
}

#[derive(Deserialize)]
pub struct GetProviderQueryParams {
    name: Option<String>,
    zip: Option<String>
}

pub async fn get_all_providers(
    Extension(database): Extension<DatabaseConnection>, 
    Query(query_params): Query<GetProviderQueryParams>
) -> Result<Json<Vec<ResponseProvider>>, StatusCode> {
    let mut zip_filter = Condition::all();
    if let Some(zip) = query_params.zip {
        zip_filter = if zip.is_empty() {
            zip_filter.add(provider::Column::ProviderZip.is_null())
        } else {
            zip_filter.add(provider::Column::ProviderZip.eq(zip))
        };
    }
    let providers = Provider::find()
        .filter(provider::Column::DeletedAt.is_null())
        // .filter(zip_filter)
        .all(&database)
        .await
        .map_err(|_error| StatusCode::INTERNAL_SERVER_ERROR)?
        .into_iter()
        .map(|db_provider| ResponseProvider {
            provider_id: db_provider.provider_id,
            provider_name: db_provider.provider_name,
            provider_zip: db_provider.provider_zip,
            provider_phone: db_provider.provider_phone,
            provider_address_1: db_provider.provider_address_1,
            provider_address_2: db_provider.provider_address_2,
            provider_contact_f_name: db_provider.provider_contact_f_name,
            provider_contact_l_name: db_provider.provider_contact_l_name,
            deleted_at: db_provider.deleted_at,
            created_by: db_provider.created_by,
        })
        .collect();

    Ok(Json(providers))

}