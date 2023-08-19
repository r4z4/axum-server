use crate::database::provider::Entity as Provider;
use axum::{
    extract::{Path},
    http::StatusCode,
    Extension, Json,
};
use sea_orm::{DatabaseConnection, EntityTrait};
use serde::{Serialize};

#[derive(Serialize)]
pub struct ResponseProvider {
    provider_id: i32,
    provider_name: String,
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
            provider_phone: provider.provider_phone,
        }))
    } else {
        Err(StatusCode::NOT_FOUND)
    }
}

pub async fn get_all_providers(Extension(database): Extension<DatabaseConnection>) -> Result<Json<Vec<ResponseProvider>>, StatusCode> {
    let providers = Provider::find()
        // .filter(priority_filter)
        // .filter(tasks::Column::DeletedAt.is_null())
        .all(&database)
        .await
        .map_err(|_error| StatusCode::INTERNAL_SERVER_ERROR)?
        .into_iter()
        .map(|db_provider| ResponseProvider {
            provider_id: db_provider.provider_id,
            provider_name: db_provider.provider_name,
            provider_phone: db_provider.provider_phone,
        })
        .collect();

    Ok(Json(providers))

}