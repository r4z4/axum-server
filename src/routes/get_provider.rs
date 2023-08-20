use crate::database::provider::{self, Entity as Provider};
use axum::{
    extract::{Path, Query, Json},
    http::StatusCode,
    Extension,
};
use sea_orm::{DatabaseConnection, EntityTrait, QueryFilter, ColumnTrait, Condition};
use serde::{Serialize, Deserialize};

#[derive(Serialize)]
pub struct ResponseProvider {
    provider_id: i32,
    provider_name: String,
    provider_zip: Option<String>,
    provider_phone: Option<String>
}

pub async fn get_provider(
    Path(provider_id): Path<i32>,
    Extension(database): Extension<DatabaseConnection>
) -> Result<Json<ResponseProvider>, StatusCode> {
    let provider = Provider::find_by_id(provider_id)
    .one(&database)
    .await
    .unwrap();

    if let Some(provider) = provider {
        Ok(Json(ResponseProvider {
            provider_id: provider.provider_id,
            provider_name: provider.provider_name,
            provider_zip: provider.provider_zip,
            provider_phone: provider.provider_phone,
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
        // .filter(priority_filter)
        // .filter(tasks::Column::DeletedAt.is_null())
        .filter(zip_filter)
        .all(&database)
        .await
        .map_err(|_error| StatusCode::INTERNAL_SERVER_ERROR)?
        .into_iter()
        .map(|db_provider| ResponseProvider {
            provider_id: db_provider.provider_id,
            provider_name: db_provider.provider_name,
            provider_zip: db_provider.provider_zip,
            provider_phone: db_provider.provider_phone,
        })
        .collect();

    Ok(Json(providers))

}