use crate::database::insurer::{self, Entity as Insurer};
use axum::{
    extract::{Path, Extension, Json},
    http::StatusCode,
};
use sea_orm::{DatabaseConnection, Set, ColumnTrait, QueryFilter, EntityTrait, IntoActiveModel};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct RequestInsurer {
    pub insurer_id: Option<i32>,
    pub insurer_name: Option<String>,
    #[serde(
        default,                                    
        skip_serializing_if = "Option::is_none",    
        with = "::serde_with::rust::double_option",
    )]
    pub insurer_email: Option<Option<String>>,
    #[serde(
        default,                                    
        skip_serializing_if = "Option::is_none",    
        with = "::serde_with::rust::double_option",
    )]
    pub insurer_address_1: Option<Option<String>>,
    #[serde(
        default,                                    
        skip_serializing_if = "Option::is_none",    
        with = "::serde_with::rust::double_option",
    )]
    pub insurer_address_2: Option<Option<String>>,
    #[serde(
        default,                                    
        skip_serializing_if = "Option::is_none",    
        with = "::serde_with::rust::double_option",
    )]
    pub insurer_zip: Option<Option<String>>,
    #[serde(
        default,                                    
        skip_serializing_if = "Option::is_none",    
        with = "::serde_with::rust::double_option",
    )]
    pub insurer_contact_f_name: Option<Option<String>>,
    #[serde(
        default,                                    
        skip_serializing_if = "Option::is_none",    
        with = "::serde_with::rust::double_option",
    )]
    pub insurer_contact_l_name: Option<Option<String>>,
    #[serde(
        default,                                    
        skip_serializing_if = "Option::is_none",    
        with = "::serde_with::rust::double_option",
    )]
    pub insurer_phone: Option<Option<String>>,
}

pub async fn partial_update_insurer(
    Path(insurer_id): Path<i32>,
    Extension(database): Extension<DatabaseConnection>,
    Json(request_insurer): Json<RequestInsurer>
) -> Result<(), StatusCode> {
    let mut db_insurer = if let Some(insurer) = Insurer::find_by_id(insurer_id)
        .one(&database)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)? 
    {
        insurer.into_active_model()
    } else {
        return Err(StatusCode::NOT_FOUND);
    };
    // Shouldn't enter this block if insurer_name set to NULL or None
    if let Some(insurer_name) = request_insurer.insurer_name {
        db_insurer.insurer_name = Set(insurer_name)
    }
    if let Some(insurer_email) = request_insurer.insurer_email {
        db_insurer.insurer_email = Set(insurer_email)
    }
    if let Some(insurer_address_1) = request_insurer.insurer_address_1 {
        db_insurer.insurer_address_1 = Set(insurer_address_1)
    }
    if let Some(insurer_address_2) = request_insurer.insurer_address_2 {
        db_insurer.insurer_address_2 = Set(insurer_address_2)
    }
    if let Some(insurer_zip) = request_insurer.insurer_zip {
        db_insurer.insurer_zip = Set(insurer_zip)
    }
    if let Some(insurer_contact_f_name) = request_insurer.insurer_contact_f_name {
        db_insurer.insurer_contact_f_name = Set(insurer_contact_f_name)
    }
    if let Some(insurer_contact_l_name) = request_insurer.insurer_contact_l_name {
        db_insurer.insurer_contact_l_name = Set(insurer_contact_l_name)
    }
    if let Some(insurer_phone) = request_insurer.insurer_phone {
        db_insurer.insurer_phone = Set(insurer_phone)
    }

    Insurer::update(db_insurer)
        .filter(insurer::Column::InsurerId.eq(insurer_id))
        .exec(&database)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(())
}