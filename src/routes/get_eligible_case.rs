use crate::database::eligible_case::Entity as EligibleCase;
use axum::{
    extract::{Path},
    http::StatusCode,
    Extension, Json,
};
use sea_orm::{DatabaseConnection, EntityTrait, prelude::Date};
use serde::{Serialize};

#[derive(Serialize)]
pub struct ResponseEligibleCase {
    eligible_case_id: i32,
    patient_id: i32,
    insurer_id: i32,
    iro_id: Option<i32>,
    provider_id: Option<i32>,
    denial_reason: Option<String>,
    expedited: Option<i32>,
    date_forwarded: Option<Date>,
    eligibility_notice: Option<Date>,
    eligible_correspondence: Option<Date>,
    insurer_notified: Option<Date>,
    decision_date: Option<Date>,
    iro_decision: Option<String>,
    file_closed: Option<Date>,
    invoice_amount: Option<f64>,
}

pub async fn get_eligible_case(
    Path(eligible_case_id): Path<i32>,
    Extension(database): Extension<DatabaseConnection>
) -> Result<Json<ResponseEligibleCase>, StatusCode> {
    let eligible_case = EligibleCase::find_by_id(eligible_case_id)
    .one(&database)
    .await
    .unwrap();

    if let Some(eligible_case) = eligible_case {
        Ok(Json(ResponseEligibleCase {
            eligible_case_id: eligible_case.eligible_case_id,
            patient_id: eligible_case.patient_id,
            insurer_id: eligible_case.insurer_id,
            iro_id: eligible_case.iro_id,
            provider_id: eligible_case.provider_id,
            denial_reason: eligible_case.denial_reason,
            expedited: eligible_case.expedited,
            date_forwarded: eligible_case.date_forwarded,
            eligibility_notice: eligible_case.eligibility_notice,
            eligible_correspondence: eligible_case.eligible_correspondence,
            insurer_notified: eligible_case.insurer_notified,
            decision_date: eligible_case.decision_date,
            iro_decision: eligible_case.iro_decision,
            file_closed: eligible_case.file_closed,
            invoice_amount: eligible_case.invoice_amount,
        }))
    } else {
        Err(StatusCode::NOT_FOUND)
    }
}

pub async fn get_all_eligible_cases(Extension(database): Extension<DatabaseConnection>) -> Result<Json<Vec<ResponseEligibleCase>>, StatusCode> {
    let eligible_cases = EligibleCase::find()
        // .filter(priority_filter)
        // .filter(tasks::Column::DeletedAt.is_null())
        .all(&database)
        .await
        .map_err(|_error| StatusCode::INTERNAL_SERVER_ERROR)?
        .into_iter()
        .map(|db_eligible_case| ResponseEligibleCase {
            eligible_case_id: db_eligible_case.eligible_case_id,
            patient_id: db_eligible_case.patient_id,
            insurer_id: db_eligible_case.insurer_id,
            iro_id: db_eligible_case.iro_id,
            provider_id: db_eligible_case.provider_id,
            denial_reason: db_eligible_case.denial_reason,
            expedited: db_eligible_case.expedited,
            date_forwarded: db_eligible_case.date_forwarded,
            eligibility_notice: db_eligible_case.eligibility_notice,
            eligible_correspondence: db_eligible_case.eligible_correspondence,
            insurer_notified: db_eligible_case.insurer_notified,
            decision_date: db_eligible_case.decision_date,
            iro_decision: db_eligible_case.iro_decision,
            file_closed: db_eligible_case.file_closed,
            invoice_amount: db_eligible_case.invoice_amount,
        })
        .collect();

    Ok(Json(eligible_cases))

}