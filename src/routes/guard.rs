use crate::{
    database::user::{self, Model, Entity as User},
    utils::jwt::{create_jwt, is_valid},
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
) -> Result<Response, StatusCode> {
    let token = request.headers().typed_get::<Authorization<Bearer>>()
        .ok_or(StatusCode::BAD_REQUEST)?
        .token()
        .to_owned();
    let database = request
        .extensions()
        .get::<DatabaseConnection>()
        .ok_or(StatusCode::INTERNAL_SERVER_ERROR)?;
    let user = User::find()
        .filter(user::Column::Token.eq(Some(token.clone())))
        .one(database)
        .await
        .map_err(|_error| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    is_valid(&token)?; // Validate token after DB call to obfuscate error type and login times
    let Some(user) = user else {return Err(StatusCode::UNAUTHORIZED)};

    request.extensions_mut().insert(user);

    Ok(next.run(request).await)
}