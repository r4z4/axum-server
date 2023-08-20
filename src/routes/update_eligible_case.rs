use crate::database::eligible_cases::{self, Entity as EligibleCase};
use axum::{
    extract::{Path, Extension, Json},
    http::StatusCode,
};
use sea_orm::{DatabaseConnection, Set, ColumnTrait, QueryFilter, EntityTrait, prelude::Date};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct RequestEligibleCase {
    pub case_id: i32,
    pub patient_id: i32,
    pub insurer_id: i32,
    pub iro_id: Option<i32>,
    pub provider_id: Option<i32>,
    pub denial_reason: Option<String>,
    pub expedited: Option<i32>,
    pub date_forwarded: Option<Date>,
    pub eligibility_notice: Option<Date>,
    pub eligible_correspondence: Option<Date>,
    pub insurer_notified: Option<Date>,
    pub decision_date: Option<Date>,
    pub iro_decision: Option<String>,
    pub file_closed: Option<Date>,
    pub invoice_amount: Option<f64>,
}

pub async fn atomic_update_eligible_case(
    Path(eligible_case_id): Path<i32>,
    Extension(database): Extension<DatabaseConnection>,
    Json(request_eligible_case): Json<RequestEligibleCase>
) -> Result<(), StatusCode> {
    let update_eligible_case = eligible_cases::ActiveModel {
        // FIXME: Change case_id => eligible_case_id
        case_id: Set(eligible_case_id),
        patient_id: Set(request_eligible_case.patient_id),
        insurer_id: Set(request_eligible_case.insurer_id),
        iro_id: Set(request_eligible_case.iro_id),
        provider_id: Set(request_eligible_case.provider_id),
        denial_reason: Set(request_eligible_case.denial_reason),
        expedited: Set(request_eligible_case.expedited),
        date_forwarded: Set(request_eligible_case.date_forwarded),
        eligibility_notice: Set(request_eligible_case.eligibility_notice),
        eligible_correspondence: Set(request_eligible_case.eligible_correspondence),
        insurer_notified: Set(request_eligible_case.insurer_notified),
        decision_date: Set(request_eligible_case.decision_date),
        iro_decision: Set(request_eligible_case.iro_decision),
        file_closed: Set(request_eligible_case.file_closed),
        invoice_amount: Set(request_eligible_case.invoice_amount),
    };

    EligibleCase::update(update_eligible_case)
        // FIXME: Change table name to singular. Change Col name to eligible_case_id
        .filter(eligible_cases::Column::CaseId.eq(eligible_case_id))
        .exec(&database)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(())
}