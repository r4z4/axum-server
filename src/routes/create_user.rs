use crate::database::user::{self, Entity as User};
use axum::{
    extract::{Path, Extension, Json},
    http::StatusCode,
};
use sea_orm::{DatabaseConnection, Set, ColumnTrait, ActiveModelTrait, QueryFilter, EntityTrait, prelude::Date, prelude::DateTimeWithTimeZone};
use serde::{Serialize, Deserialize};

#[derive(Deserialize)]
pub struct RequestUser {
    username: String,
    password: String,
}

#[derive(Serialize)]
pub struct ResponseUser {
    username: String,
    // We'll give them their ID (for now)
    id: i32,
    token: String,
}

#[axum_macros::debug_handler]
pub async fn create_user(
    Extension(database): Extension<DatabaseConnection>,
    Json(request_user): Json<RequestUser>
) -> Result<Json<ResponseUser>, StatusCode> {
    let now = chrono::Utc::now();
    let new_user = user::ActiveModel{ 
        username: Set(request_user.username),
        password: Set(request_user.password),
        token: Set(Some("asdfasd3453".to_owned())),
        created_at: Set(now.into()),
        ..Default::default()
     }
     .save(&database)
     .await
     .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

     Ok(Json(ResponseUser {
        username: new_user.username.unwrap(), 
        id: new_user.user_id.unwrap(), 
        token: new_user.token.unwrap().unwrap(), 
     }))
}