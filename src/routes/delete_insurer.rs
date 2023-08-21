use crate::database::insurer::{self, Entity as Insurer};
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

pub async fn delete_insurer(
    Path(insurer_id): Path<i32>,
    Extension(database): Extension<DatabaseConnection>,
    Query(query_params): Query<QueryParams>
) -> Result<(), StatusCode> {
    let mut insurer = if let Some(insurer) = Insurer::find_by_id(insurer_id)
        .one(&database)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)? 
        {
            insurer.into_active_model()
        } else {
            return Err(StatusCode::NOT_FOUND);
        };

    if query_params.soft {
        let now = chrono::Utc::now();
        insurer.deleted_at = Set(Some(now.into()));
        Insurer::update(insurer)
            .exec(&database)
            .await
            .map_err(|_error| StatusCode::INTERNAL_SERVER_ERROR)?;
    } else {
        Insurer::delete(insurer)
            .exec(&database)
            .await
            .map_err(|_error| StatusCode::INTERNAL_SERVER_ERROR)?;
    }

    // Insurer::delete_many()
    //     .filter(insurer::Column:InsurerId.eq(insurer_id))
    //     .exec(&database)
    //     .await
    //     .map_err(|_error| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(())
}