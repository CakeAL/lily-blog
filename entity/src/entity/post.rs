//! `SeaORM` Entity, @generated by sea-orm-codegen 1.0.0

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "post")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub title: String,
    pub summary: String,
    pub md_path: String,
    pub html_path: String,
    pub hit: i32,
    pub words_len: Option<i32>,
    pub is_del: bool,
    pub publish_time: DateTimeWithTimeZone,
    pub update_time: Option<DateTimeWithTimeZone>,
    #[sea_orm(column_type = "VarBinary(StringLen::None)", nullable)]
    pub tag_id: Option<Vec<u8>>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::comment::Entity")]
    Comment,
}

impl Related<super::comment::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Comment.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
