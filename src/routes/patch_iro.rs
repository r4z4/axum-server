use crate::database::iro::{self, Entity as Iro};
use axum::{
    extract::{Path, Extension, Json},
    http::StatusCode,
};
use sea_orm::{DatabaseConnection, Set, ColumnTrait, QueryFilter, EntityTrait, IntoActiveModel, prelude::Date};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct RequestIro {
    pub iro_id: Option<i32>,
    pub iro_name: Option<String>,
    #[serde(
        default,                                    
        skip_serializing_if = "Option::is_none",    
        with = "::serde_with::rust::double_option",
    )]
    pub iro_email: Option<Option<String>>,
    #[serde(
        default,                                    
        skip_serializing_if = "Option::is_none",    
        with = "::serde_with::rust::double_option",
    )]
    pub iro_address_1: Option<Option<String>>,
    #[serde(
        default,                                    
        skip_serializing_if = "Option::is_none",    
        with = "::serde_with::rust::double_option",
    )]
    pub iro_address_2: Option<Option<String>>,
    #[serde(
        default,                                    
        skip_serializing_if = "Option::is_none",    
        with = "::serde_with::rust::double_option",
    )]
    pub iro_zip: Option<Option<String>>,
    #[serde(
        default,                                    
        skip_serializing_if = "Option::is_none",    
        with = "::serde_with::rust::double_option",
    )]
    pub iro_contact_f_name: Option<Option<String>>,
    #[serde(
        default,                                    
        skip_serializing_if = "Option::is_none",    
        with = "::serde_with::rust::double_option",
    )]
    pub iro_contact_l_name: Option<Option<String>>,
    #[serde(
        default,                                    
        skip_serializing_if = "Option::is_none",    
        with = "::serde_with::rust::double_option",
    )]
    pub iro_license_expiration: Option<Option<Date>>,
}

pub async fn partial_update_iro(
    Path(iro_id): Path<i32>,
    Extension(database): Extension<DatabaseConnection>,
    Json(request_iro): Json<RequestIro>
) -> Result<(), StatusCode> {
    let mut db_iro = if let Some(iro) = Iro::find_by_id(iro_id)
        .one(&database)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)? 
    {
        iro.into_active_model()
    } else {
        return Err(StatusCode::NOT_FOUND);
    };
    // Shouldn't enter this block if iro_name set to NULL or None
    if let Some(iro_name) = request_iro.iro_name {
        db_iro.iro_name = Set(iro_name)
    }
    if let Some(iro_email) = request_iro.iro_email {
        db_iro.iro_email = Set(iro_email)
    }
    if let Some(iro_address_1) = request_iro.iro_address_1 {
        db_iro.iro_address_1 = Set(iro_address_1)
    }
    if let Some(iro_address_2) = request_iro.iro_address_2 {
        db_iro.iro_address_2 = Set(iro_address_2)
    }
    if let Some(iro_zip) = request_iro.iro_zip {
        db_iro.iro_zip = Set(iro_zip)
    }
    if let Some(iro_contact_f_name) = request_iro.iro_contact_f_name {
        db_iro.iro_contact_f_name = Set(iro_contact_f_name)
    }
    if let Some(iro_contact_l_name) = request_iro.iro_contact_l_name {
        db_iro.iro_contact_l_name = Set(iro_contact_l_name)
    }
    if let Some(iro_license_expiration) = request_iro.iro_license_expiration {
        db_iro.iro_license_expiration = Set(iro_license_expiration)
    }

    Iro::update(db_iro)
        .filter(iro::Column::IroId.eq(iro_id))
        .exec(&database)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(())
}