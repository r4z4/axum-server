use crate::database::eligible_cases::{self, Entity as EligibleCase};
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

pub async fn delete_eligible_case(
    Path(eligible_case_id): Path<i32>,
    Extension(database): Extension<DatabaseConnection>,
    Query(query_params): Query<QueryParams>
) -> Result<(), StatusCode> {
    let mut eligible_case = if let Some(eligible_case) = EligibleCase::find_by_id(eligible_case_id)
        .one(&database)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)? 
        {
            eligible_case.into_active_model()
        } else {
            return Err(StatusCode::NOT_FOUND);
        };

    if query_params.soft {
        let now = chrono::Utc::now();
        eligible_case.deleted_at = Set(Some(now.into()));
        EligibleCase::update(eligible_case)
            .exec(&database)
            .await
            .map_err(|_error| StatusCode::INTERNAL_SERVER_ERROR)?;
    } else {
        EligibleCase::delete(eligible_case)
            .exec(&database)
            .await
            .map_err(|_error| StatusCode::INTERNAL_SERVER_ERROR)?;
    }

    // EligibleCase::delete_many()
    //     .filter(eligible_case::Column:EligibleCaseId.eq(eligible_case_id))
    //     .exec(&database)
    //     .await
    //     .map_err(|_error| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(())
}