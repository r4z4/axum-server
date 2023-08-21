use crate::database::eligible_cases::{self, Entity as EligibleCase};
use axum::{
    extract::{Path, Extension, Json},
    http::StatusCode,
};
use sea_orm::{DatabaseConnection, Set, ColumnTrait, QueryFilter, EntityTrait, IntoActiveModel, prelude::Date};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct RequestEligibleCase {
    pub case_id: Option<i32>,
    pub patient_id: Option<i32>,
    pub insurer_id: Option<i32>,
    pub iro_id: Option<Option<i32>>,
    pub provider_id: Option<Option<i32>>,
    #[serde(
        default,                                    
        skip_serializing_if = "Option::is_none",    
        with = "::serde_with::rust::double_option",
    )]
    pub denial_reason: Option<Option<String>>,
    #[serde(
        default,                                    
        skip_serializing_if = "Option::is_none",    
        with = "::serde_with::rust::double_option",
    )]
    pub expedited: Option<Option<i32>>,
    #[serde(
        default,                                    
        skip_serializing_if = "Option::is_none",    
        with = "::serde_with::rust::double_option",
    )]
    pub date_forwarded: Option<Option<Date>>,
    #[serde(
        default,                                    
        skip_serializing_if = "Option::is_none",    
        with = "::serde_with::rust::double_option",
    )]
    pub eligibility_notice: Option<Option<Date>>,
    #[serde(
        default,                                    
        skip_serializing_if = "Option::is_none",    
        with = "::serde_with::rust::double_option",
    )]
    pub eligible_correspondence: Option<Option<Date>>,
    #[serde(
        default,                                    
        skip_serializing_if = "Option::is_none",    
        with = "::serde_with::rust::double_option",
    )]
    pub insurer_notified: Option<Option<Date>>,
    #[serde(
        default,                                    
        skip_serializing_if = "Option::is_none",    
        with = "::serde_with::rust::double_option",
    )]
    pub decision_date: Option<Option<Date>>,
    #[serde(
        default,                                    
        skip_serializing_if = "Option::is_none",    
        with = "::serde_with::rust::double_option",
    )]
    pub iro_decision: Option<Option<String>>,
    #[serde(
        default,                                    
        skip_serializing_if = "Option::is_none",    
        with = "::serde_with::rust::double_option",
    )]
    pub file_closed: Option<Option<Date>>,
    #[serde(
        default,                                    
        skip_serializing_if = "Option::is_none",    
        with = "::serde_with::rust::double_option",
    )]
    pub invoice_amount: Option<Option<f64>>,
}

pub async fn partial_update_eligible_case(
    Path(eligible_case_id): Path<i32>,
    Extension(database): Extension<DatabaseConnection>,
    Json(request_eligible_case): Json<RequestEligibleCase>
) -> Result<(), StatusCode> {
    let mut db_eligible_case = if let Some(eligible_case) = EligibleCase::find_by_id(eligible_case_id)
        .one(&database)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)? 
    {
        eligible_case.into_active_model()
    } else {
        return Err(StatusCode::NOT_FOUND);
    };
    // Shouldn't enter this block if eligible_case_name set to NULL or None
    if let Some(patient_id) = request_eligible_case.patient_id {
        db_eligible_case.patient_id = Set(patient_id)
    }
    if let Some(insurer_id) = request_eligible_case.insurer_id {
        db_eligible_case.insurer_id = Set(insurer_id)
    }
    if let Some(iro_id) = request_eligible_case.iro_id {
        db_eligible_case.iro_id = Set(iro_id)
    }
    if let Some(provider_id) = request_eligible_case.provider_id {
        db_eligible_case.provider_id = Set(provider_id)
    }
    if let Some(denial_reason) = request_eligible_case.denial_reason {
        db_eligible_case.denial_reason = Set(denial_reason)
    }
    if let Some(expedited) = request_eligible_case.expedited {
        db_eligible_case.expedited = Set(expedited)
    }
    if let Some(date_forwarded) = request_eligible_case.date_forwarded {
        db_eligible_case.date_forwarded = Set(date_forwarded)
    }
    if let Some(eligibility_notice) = request_eligible_case.eligibility_notice {
        db_eligible_case.eligibility_notice = Set(eligibility_notice)
    }
    if let Some(eligible_correspondence) = request_eligible_case.eligible_correspondence {
        db_eligible_case.eligible_correspondence = Set(eligible_correspondence)
    }
    if let Some(insurer_notified) = request_eligible_case.insurer_notified {
        db_eligible_case.insurer_notified = Set(insurer_notified)
    }
    if let Some(decision_date) = request_eligible_case.decision_date {
        db_eligible_case.decision_date = Set(decision_date)
    }
    if let Some(iro_decision) = request_eligible_case.iro_decision {
        db_eligible_case.iro_decision = Set(iro_decision)
    }
    if let Some(file_closed) = request_eligible_case.file_closed {
        db_eligible_case.file_closed = Set(file_closed)
    }
    if let Some(invoice_amount) = request_eligible_case.invoice_amount {
        db_eligible_case.invoice_amount = Set(invoice_amount)
    }

    EligibleCase::update(db_eligible_case)
        .filter(eligible_cases::Column::CaseId.eq(eligible_case_id))
        .exec(&database)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(())
}