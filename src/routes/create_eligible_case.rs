use crate::database::eligible_case;
use axum::{
    extract::{Extension, Json},
    http::StatusCode,
};
use sea_orm::{DatabaseConnection, Set, prelude::Date, ActiveModelTrait};
use serde::{Serialize, Deserialize};

#[derive(Deserialize)]
pub struct RequestEligibleCase {
    patient_id: i32,
    insurer_id: i32,
    iro_id: Option<i32>,
    provider_id: Option<i32>,
    denial_reason: Option<String>,
    expedited: Option<i32>,
    date_forwarded: Option<String>,
}

#[axum_macros::debug_handler]
pub async fn create_eligible_case(
    Extension(database): Extension<DatabaseConnection>,
    Json(request_eligible_case): Json<RequestEligibleCase> 
) {
    // let split: Vec<&str> = request_eligible_case.date_forwarded.split("-").collect();
    let default_date = "2024-10-11";
    let input_year: i32 = request_eligible_case.date_forwarded.as_ref().unwrap_or(&default_date.to_owned()).split("-").nth(0).unwrap().parse::<i32>().unwrap();
    let input_month: u32  = request_eligible_case.date_forwarded.as_ref().unwrap_or(&default_date.to_owned()).split("-").nth(1).unwrap().parse::<u32>().unwrap();
    let input_day: u32  = request_eligible_case.date_forwarded.as_ref().unwrap_or(&default_date.to_owned()).split("-").nth(2).unwrap().parse::<u32>().unwrap();
    let new_eligible_case = eligible_case::ActiveModel{ 
        // Some() is for Option<>
        patient_id: Set(request_eligible_case.patient_id),
        insurer_id: Set(request_eligible_case.insurer_id),
        iro_id: Set(request_eligible_case.iro_id),
        provider_id: Set(request_eligible_case.provider_id),
        denial_reason: Set(request_eligible_case.denial_reason),
        expedited: Set(request_eligible_case.expedited),
        // date_forwarded: Set(Some(Date::from_ymd_opt(2023, 1, 3).unwrap())),
        date_forwarded: Set(Some(Date::from_ymd_opt(input_year, input_month, input_day).unwrap())),
        eligibility_notice: Set(Some(Date::from_ymd_opt(2023, 1, 3).unwrap())),
        eligible_correspondence: Set(Some(Date::from_ymd_opt(2023, 1, 3).unwrap())),
        insurer_notified: Set(Some(Date::from_ymd_opt(2023, 1, 3).unwrap())),
        decision_date: Set(Some(Date::from_ymd_opt(2023, 1, 10).unwrap())),
        iro_decision: Set(Some("Upheld".to_owned())),
        file_closed: Set(Some(Date::from_ymd_opt(2023, 1, 10).unwrap())),
        invoice_amount: Set(Some(300.50)),
        ..Default::default()
     };

     let result = new_eligible_case.save(&database).await.unwrap();

     dbg!(result);
}