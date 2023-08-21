use crate::database::provider::{self, Entity as Provider};
use axum::{
    extract::{Path, Extension},
    http::StatusCode,
};
use sea_orm::{DatabaseConnection, EntityTrait, IntoActiveModel};

pub async fn delete_provider(
    Path(provider_id): Path<i32>,
    Extension(database): Extension<DatabaseConnection>
) -> Result<(), StatusCode> {
    let provider = if let Some(provider) = Provider::find_by_id(provider_id)
        .one(&database)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)? 
        {
            provider.into_active_model()
        } else {
            return Err(StatusCode::NOT_FOUND);
        };

    Provider::delete(provider)
        .exec(&database)
        .await
        .map_err(|_error| StatusCode::INTERNAL_SERVER_ERROR)?;

    // Provider::delete_many()
    //     .filter(provider::Column:ProviderId.eq(provider_id))
    //     .exec(&database)
    //     .await
    //     .map_err(|_error| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(())
}