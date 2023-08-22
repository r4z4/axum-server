use crate::{
    database::user::{self, Model, Entity as User},
    utils::jwt::{create_jwt, is_valid},
};
use axum::{
    headers::{authorization::Bearer, Authorization},
    extract::{Extension, Json},
    http::StatusCode,
};
use sea_orm::{DatabaseConnection, Set, ColumnTrait, ActiveModelTrait, QueryFilter, EntityTrait, IntoActiveModel};
use serde::{Serialize, Deserialize};
use uuid::Uuid;

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
    token: Option<String>,
}

#[axum_macros::debug_handler]
pub async fn get_all_users(
    Extension(database): Extension<DatabaseConnection>
) -> Result<Json<Vec<ResponseUser>>, StatusCode> {
    let users = User::find()
        .filter(user::Column::DeletedAt.is_null())
        // .filter(zip_filter)
        .all(&database)
        .await
        .map_err(|_error| StatusCode::INTERNAL_SERVER_ERROR)?
        .into_iter()
        .map(|db_user| ResponseUser {
            username: db_user.username,
            id: db_user.user_id,
            token: Some(db_user.token.unwrap_or_default())
        })
        .collect();

    Ok(Json(users))

}

#[axum_macros::debug_handler]
pub async fn create_user(
    Extension(database): Extension<DatabaseConnection>,
    Json(request_user): Json<RequestUser>
) -> Result<Json<ResponseUser>, StatusCode> {
    let now = chrono::Utc::now();
    let jwt = create_jwt()?;
    let new_user = user::ActiveModel{ 
        username: Set(request_user.username),
        password: Set(hash_password(request_user.password)?),
        token: Set(Some(jwt)),
        created_at: Set(now.into()),
        ..Default::default()
     }
     .save(&database)
     .await
     .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

     Ok(Json(ResponseUser {
        username: new_user.username.unwrap(), 
        id: new_user.user_id.unwrap(), 
        token: Some(new_user.token.unwrap().unwrap_or_default()),
     }))
}

pub async fn login(
    Extension(database): Extension<DatabaseConnection>,
    Json(request_user): Json<RequestUser>
) -> Result<Json<ResponseUser>, StatusCode> {
    let db_user = User::find()
        .filter(user::Column::Username.eq(request_user.username))
        .one(&database)
        .await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if let Some(db_user) = db_user {
        if !verify_password(request_user.password, &db_user.password)? {
            return Err(StatusCode::UNAUTHORIZED);
        }

        let new_jwt = create_jwt()?;
        let mut user = db_user.into_active_model();

        user.token = Set(Some(new_jwt));
        let saved_user = user.save(&database)
            .await
            .map_err(|_error| StatusCode::INTERNAL_SERVER_ERROR)?;

        Ok(Json(ResponseUser {
            username: saved_user.username.unwrap(),
            id: saved_user.user_id.unwrap(),
            token: Some(saved_user.token.unwrap().unwrap()),
        }))
    } else {
        Err(StatusCode::NOT_FOUND)
    }
}

pub async fn logout(
    Extension(database): Extension<DatabaseConnection>,
    Extension(user): Extension<Model>,
) -> Result<(), StatusCode> {
    let mut user = user.into_active_model();
    user.token = Set(None);
    user.save(&database)
        .await
        .map_err(|_error| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(())
}

fn hash_password(password: String) -> Result<String, StatusCode> {
    bcrypt::hash(password, 14).map_err(|_error| StatusCode::INTERNAL_SERVER_ERROR)
}

fn verify_password(password: String, hash: &str) -> Result<bool, StatusCode> {
    bcrypt::verify(password, hash).map_err(|_error| StatusCode::INTERNAL_SERVER_ERROR)
}