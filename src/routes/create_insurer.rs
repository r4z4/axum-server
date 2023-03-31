use crate::database::insurer;
use axum::{Extension};
use sea_orm::{DatabaseConnection, Set};
use sea_orm::{ActiveModelTrait};

pub async fn create_insurer(Extension(database): Extension<DatabaseConnection>) {
    let new_insurer = insurer::ActiveModel{ 
        insurer_name: Set("Main Insurer".to_owned()),
        insurer_phone: Set(Some("402-111-1111".to_owned())),
        insurer_email: Set(Some("main_insurer@insurer.com".to_owned())),
        ..Default::default()
     };

     let result = new_insurer.save(&database).await.unwrap();

     dbg!(result);
}