use crate::database::iro::{self, Entity as Iro};
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

pub async fn delete_iro(
    Path(iro_id): Path<i32>,
    Extension(database): Extension<DatabaseConnection>,
    Query(query_params): Query<QueryParams>
) -> Result<(), StatusCode> {
    let mut iro = if let Some(iro) = Iro::find_by_id(iro_id)
        .one(&database)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)? 
        {
            iro.into_active_model()
        } else {
            return Err(StatusCode::NOT_FOUND);
        };

    if query_params.soft {
        let now = chrono::Utc::now();
        iro.deleted_at = Set(Some(now.into()));
        Iro::update(iro)
            .exec(&database)
            .await
            .map_err(|_error| StatusCode::INTERNAL_SERVER_ERROR)?;
    } else {
        Iro::delete(iro)
            .exec(&database)
            .await
            .map_err(|_error| StatusCode::INTERNAL_SERVER_ERROR)?;
    }

    // Iro::delete_many()
    //     .filter(iro::Column:IroId.eq(iro_id))
    //     .exec(&database)
    //     .await
    //     .map_err(|_error| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(())
}