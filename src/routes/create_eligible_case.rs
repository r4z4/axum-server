use crate::database::eligible_case;
use axum::{Extension};
use sea_orm::{DatabaseConnection, Set};
use sea_orm::{ActiveModelTrait};

pub async fn create_eligible_case(Extension(database): Extension<DatabaseConnection>) {
    let new_eligible_case = eligible_case::ActiveModel{ 
        ..Default::default()
     };

     let result = new_eligible_case.save(&database).await.unwrap();

     dbg!(result);
}