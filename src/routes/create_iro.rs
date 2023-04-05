use crate::database::iro;
use axum::{Extension};
use sea_orm::{DatabaseConnection, Set};
use sea_orm::{ActiveModelTrait};

pub async fn create_iro(Extension(database): Extension<DatabaseConnection>) {
    let new_iro = iro::ActiveModel{ 
        iro_name: Set("Main IRO".to_owned()),
        iro_email: Set(Some("main_iro@iro.com".to_owned())),
        ..Default::default()
     };

     let result = new_iro.save(&database).await.unwrap();

     dbg!(result);
}