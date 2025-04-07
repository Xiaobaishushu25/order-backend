use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "dish")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: String,
    pub index: i32,
    #[sea_orm(unique)]
    pub name: String,
    pub price:f64,
    pub picture:String,
    pub status:Status,
    pub created_at:String,
}

#[derive(Clone,Debug,Serialize,Deserialize,PartialEq,DeriveActiveEnum,EnumIter)]
#[sea_orm(rs_type = "String", db_type = "String(StringLen::None)")]
pub enum Status {
    #[sea_orm(string_value = "normal")]
    Normal, // 正常
    #[sea_orm(string_value = "delist")]
    Delist, // 已下架
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::category_dish_map::Entity")]
    CategoryDishMap
}
impl Related<super::category_dish_map::Entity> for Entity {
    fn to() -> RelationDef {
        crate::entities::category::Relation::CategoryDishMap.def()
    }
}
impl ActiveModelBehavior for ActiveModel {}