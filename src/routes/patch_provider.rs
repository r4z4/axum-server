use crate::database::provider::{self, Entity as Provider};
use axum::{
    extract::{Path, Extension, Json},
    http::StatusCode,
};
use sea_orm::{DatabaseConnection, Set, ColumnTrait, QueryFilter, EntityTrait, IntoActiveModel};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct RequestProvider {
    pub provider_id: Option<i32>,
    pub provider_name: Option<String>,
    #[serde(
        default,                                    
        skip_serializing_if = "Option::is_none",    
        with = "::serde_with::rust::double_option",
    )]
    pub provider_address_1: Option<Option<String>>,
    #[serde(
        default,                                    
        skip_serializing_if = "Option::is_none",    
        with = "::serde_with::rust::double_option",
    )]
    pub provider_address_2: Option<Option<String>>,
    #[serde(
        default,                                    
        skip_serializing_if = "Option::is_none",    
        with = "::serde_with::rust::double_option",
    )]
    pub provider_zip: Option<Option<String>>,
    #[serde(
        default,                                    
        skip_serializing_if = "Option::is_none",    
        with = "::serde_with::rust::double_option",
    )]
    pub provider_contact_f_name: Option<Option<String>>,
    #[serde(
        default,                                    
        skip_serializing_if = "Option::is_none",    
        with = "::serde_with::rust::double_option",
    )]
    pub provider_contact_l_name: Option<Option<String>>,
    #[serde(
        default,                                    
        skip_serializing_if = "Option::is_none",    
        with = "::serde_with::rust::double_option",
    )]
    pub provider_phone: Option<Option<String>>,
}

pub async fn partial_update_provider(
    Path(provider_id): Path<i32>,
    Extension(database): Extension<DatabaseConnection>,
    Json(request_provider): Json<RequestProvider>
) -> Result<(), StatusCode> {
    let mut db_provider = if let Some(provider) = Provider::find_by_id(provider_id)
        .one(&database)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)? 
    {
        provider.into_active_model()
    } else {
        return Err(StatusCode::NOT_FOUND);
    };
    // Shouldn't enter this block if provider_name set to NULL or None
    if let Some(provider_name) = request_provider.provider_name {
        db_provider.provider_name = Set(provider_name)
    }
    if let Some(provider_address_1) = request_provider.provider_address_1 {
        db_provider.provider_address_1 = Set(provider_address_1)
    }
    if let Some(provider_address_2) = request_provider.provider_address_2 {
        db_provider.provider_address_2 = Set(provider_address_2)
    }
    if let Some(provider_zip) = request_provider.provider_zip {
        db_provider.provider_zip = Set(provider_zip)
    }
    if let Some(provider_contact_f_name) = request_provider.provider_contact_f_name {
        db_provider.provider_contact_f_name = Set(provider_contact_f_name)
    }
    if let Some(provider_contact_l_name) = request_provider.provider_contact_l_name {
        db_provider.provider_contact_l_name = Set(provider_contact_l_name)
    }
    if let Some(provider_phone) = request_provider.provider_phone {
        db_provider.provider_phone = Set(provider_phone)
    }

    Provider::update(db_provider)
        .filter(provider::Column::ProviderId.eq(provider_id))
        .exec(&database)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(())
}