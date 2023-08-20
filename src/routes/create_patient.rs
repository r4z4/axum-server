use crate::database::patient;
use axum::{Extension};
use sea_orm::{DatabaseConnection, Set};
use sea_orm::{ActiveModelTrait};
use sea_orm::entity::prelude::{Date};

pub async fn create_patient(Extension(database): Extension<DatabaseConnection>) {
    let new_patient = patient::ActiveModel{ 
        patient_f_name: Set("Robert".to_owned()),
        patient_l_name: Set("Hoyt".to_owned()),
        patient_email: Set(Some("rob_hoyt@patient.com".to_owned())),
        patient_address_1: Set(Some("2323 South 78th St.".to_owned())),
        patient_address_2: Set(Some("Apt. 77".to_owned())),
        patient_zip: Set(Some("68512".to_owned())),
        patient_dob: Set(Some(Date::from_ymd_opt(1955, 5, 17).unwrap())),
        ..Default::default()
     };

     let result = new_patient.save(&database).await.unwrap();

     dbg!(result);
}