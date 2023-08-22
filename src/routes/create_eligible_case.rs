use crate::database::eligible_case;
use axum::{Extension};
use sea_orm::{DatabaseConnection, Set};
use sea_orm::{ActiveModelTrait};
use sea_orm::entity::prelude::{Date};

pub async fn create_eligible_case(Extension(database): Extension<DatabaseConnection>) {
    let new_eligible_case = eligible_case::ActiveModel{ 
        // Some() is for Option<>
        patient_id: Set(1),
        insurer_id: Set(1),
        iro_id: Set(Some(1)),
        provider_id: Set(Some(1)),
        denial_reason: Set(Some("Cosmetic Operation".to_owned())),
        expedited: Set(Some(0)),
        date_forwarded: Set(Some(Date::from_ymd_opt(2023, 1, 3).unwrap())),
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