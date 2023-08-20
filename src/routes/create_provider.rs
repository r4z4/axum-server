use crate::database::provider;
use axum::{Extension};
use sea_orm::{DatabaseConnection, Set};
use sea_orm::{ActiveModelTrait};

pub async fn create_provider(Extension(database): Extension<DatabaseConnection>) {
    let new_provider = provider::ActiveModel{ 
        provider_name: Set("Main Provider".to_owned()),
        provider_phone: Set(Some("402-333-4444".to_owned())),
        provider_zip: Set(Some("68124".to_owned())),
        ..Default::default()
     };

     let result = new_provider.save(&database).await.unwrap();

     dbg!(result);
}