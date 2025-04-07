use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "category")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: String,
    pub index: i32,
    #[sea_orm(unique)]
    pub name: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::category_dish_map::Entity")]
    CategoryDishMap
}
impl Related<super::category_dish_map::Entity> for Entity{
    fn to() -> RelationDef {
        Relation::CategoryDishMap.def()
    }
}
impl ActiveModelBehavior for ActiveModel {}