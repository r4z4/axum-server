use crate::database::provider;
use crate::database::user::{self, Entity as User};
use axum::{
    headers::{authorization::Bearer, Authorization},
    extract::{Extension, Json},
    http::StatusCode, TypedHeader
};
use sea_orm::{DatabaseConnection, Set, ColumnTrait, EntityTrait, ActiveModelTrait, QueryFilter};
use serde::{Serialize, Deserialize};

#[derive(Deserialize)]
pub struct RequestProvider {
    provider_name: String,
    provider_phone: Option<String>,
    provider_zip: Option<String>,
    provider_address_1: Option<String>,
    provider_address_2: Option<String>,
    provider_contact_f_name: Option<String>,
    provider_contact_l_name: Option<String>,
}

#[axum_macros::debug_handler]
pub async fn create_provider(
    Extension(database): Extension<DatabaseConnection>,
    authorization: TypedHeader<Authorization<Bearer>>,
    Json(request_provider): Json<RequestProvider> 
) -> Result<(), StatusCode> {
    let token = authorization.token();

    let user = if let Some(user) = User::find()
        .filter(user::Column::Token.eq(Some(token)))
        .one(&database)
        .await
        .map_err(|_error| StatusCode::INTERNAL_SERVER_ERROR)? 
    {
        user
    } else {
        return Err(StatusCode::UNAUTHORIZED);
    };

    let new_provider = provider::ActiveModel{ 
        provider_name: Set(request_provider.provider_name),
        provider_phone: Set(request_provider.provider_phone),
        provider_zip: Set(request_provider.provider_zip),
        provider_address_1: Set(request_provider.provider_address_1),
        provider_address_2: Set(request_provider.provider_address_2),
        provider_contact_f_name: Set(request_provider.provider_contact_f_name),
        provider_contact_l_name: Set(request_provider.provider_contact_l_name),
        created_by: Set(Some(user.user_id)),
        ..Default::default()
     };

     let _result = new_provider.save(&database).await.unwrap();

     Ok(())
}