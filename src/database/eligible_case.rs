//! `SeaORM` Entity. Generated by sea-orm-codegen 0.11.1

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "eligible_case")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub eligible_case_id: i32,
    pub patient_id: i32,
    pub insurer_id: i32,
    pub iro_id: Option<i32>,
    pub provider_id: Option<i32>,
    pub denial_reason: Option<String>,
    pub expedited: Option<i32>,
    pub date_forwarded: Option<Date>,
    pub eligibility_notice: Option<Date>,
    pub eligible_correspondence: Option<Date>,
    pub insurer_notified: Option<Date>,
    pub decision_date: Option<Date>,
    pub iro_decision: Option<String>,
    pub file_closed: Option<Date>,
    #[sea_orm(column_type = "Double", nullable)]
    pub invoice_amount: Option<f64>,
    pub deleted_at: Option<DateTimeWithTimeZone>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::insurer::Entity",
        from = "Column::InsurerId",
        to = "super::insurer::Column::InsurerId",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    Insurer,
    #[sea_orm(
        belongs_to = "super::iro::Entity",
        from = "Column::IroId",
        to = "super::iro::Column::IroId",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    Iro,
    #[sea_orm(
        belongs_to = "super::patient::Entity",
        from = "Column::PatientId",
        to = "super::patient::Column::PatientId",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    Patient,
    #[sea_orm(
        belongs_to = "super::provider::Entity",
        from = "Column::ProviderId",
        to = "super::provider::Column::ProviderId",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    Provider,
}

impl Related<super::insurer::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Insurer.def()
    }
}

impl Related<super::iro::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Iro.def()
    }
}

impl Related<super::patient::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Patient.def()
    }
}

impl Related<super::provider::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Provider.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
