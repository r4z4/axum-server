use crate::database::patient;
use axum::{Extension};
use sea_orm::{DatabaseConnection, Set};
use sea_orm::{ActiveModelTrait};

pub async fn create_patient(Extension(database): Extension<DatabaseConnection>) {
    let new_patient = patient::ActiveModel{ 
        patient_f_name: Set("Robert".to_owned()),
        patient_l_name: Set("Hoyt".to_owned()),
        patient_email: Set(Some("rob_hoyt@patient.com".to_owned())),
        ..Default::default()
     };

     let result = new_patient.save(&database).await.unwrap();

     dbg!(result);
}