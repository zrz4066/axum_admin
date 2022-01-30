//! SeaORM Entity. Generated by sea-orm-codegen 0.5.0

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Default, Debug, DeriveEntity)]
pub struct Entity;

impl EntityName for Entity {
    fn table_name(&self) -> &str {
        "sys_role"
    }
}

#[derive(Clone, Debug, PartialEq, DeriveModel, DeriveActiveModel, Serialize, Deserialize)]
pub struct Model {
    pub role_id: String,
    pub role_name: String,
    pub role_key: String,
    pub list_order: i32,
    pub data_scope: String,
    pub status: String,
    pub remark: String,
    pub created_at: Option<DateTime>,
    pub updated_at: Option<DateTime>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveColumn)]
pub enum Column {
    RoleId,
    RoleName,
    RoleKey,
    ListOrder,
    DataScope,
    Status,
    Remark,
    CreatedAt,
    UpdatedAt,
}

#[derive(Copy, Clone, Debug, EnumIter, DerivePrimaryKey)]
pub enum PrimaryKey {
    RoleId,
}

impl PrimaryKeyTrait for PrimaryKey {
    type ValueType = String;
    fn auto_increment() -> bool {
        false
    }
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {}

impl ColumnTrait for Column {
    type EntityName = Entity;
    fn def(&self) -> ColumnDef {
        match self {
            Self::RoleId => ColumnType::String(Some(32u32)).def(),
            Self::RoleName => ColumnType::String(Some(20u32)).def(),
            Self::RoleKey => ColumnType::String(Some(100u32)).def(),
            Self::ListOrder => ColumnType::Integer.def(),
            Self::DataScope => ColumnType::Char(Some(1u32)).def(),
            Self::Status => ColumnType::Char(Some(1u32)).def(),
            Self::Remark => ColumnType::String(Some(255u32)).def(),
            Self::CreatedAt => ColumnType::DateTime.def().null(),
            Self::UpdatedAt => ColumnType::DateTime.def().null(),
        }
    }
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        panic!("No RelationDef")
    }
}

impl ActiveModelBehavior for ActiveModel {}
