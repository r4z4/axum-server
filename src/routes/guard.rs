use crate::{
    database::user::{self, Model, Entity as User},
    utils::jwt::{create_jwt, is_valid},
    utils::app_error::*,
};
use axum::{
    headers::{authorization::Bearer, Authorization, HeaderMapExt},
    extract::{Extension, Json},
    http::{Request, StatusCode},
    middleware::Next,
    response::Response,
};
use sea_orm::{DatabaseConnection, QueryFilter, ColumnTrait, Set, EntityTrait, IntoActiveModel};

pub async fn guard<T>(
    mut request: Request<T>,
    next: Next<T>,
) -> Result<Response, AppError> {
    let token = request.headers().typed_get::<Authorization<Bearer>>()
        .ok_or_else(|| AppError { code: StatusCode::BAD_REQUEST, message: "Missing Bearer Token".to_string() })?
        .token()
        .to_owned();
    let database = request
        .extensions()
        .get::<DatabaseConnection>()
        .ok_or_else(|| AppError { code: StatusCode::INTERNAL_SERVER_ERROR, message: "Internal Server Error".to_string() })?;
    let user = User::find()
        .filter(user::Column::Token.eq(Some(token.clone())))
        .one(database)
        .await
        .map_err(|_error| AppError { code: StatusCode::INTERNAL_SERVER_ERROR, message: "Internal Server Error".to_string() })?;
    
    is_valid(&token)?; // Validate token after DB call to obfuscate error type and login times
    let Some(user) = user else {return Err(AppError { code: StatusCode::UNAUTHORIZED, message: "You are not authorized. Please login or create an account.".to_string() })};

    request.extensions_mut().insert(user);

    Ok(next.run(request).await)
}