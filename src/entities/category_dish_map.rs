use sea_orm::{EntityTrait, Linked, Related, RelationDef, RelationTrait};
use sea_orm::PrimaryKeyTrait;
use sea_orm::DerivePrimaryKey;
use sea_orm::{ActiveModelBehavior, DeriveEntityModel, DeriveRelation, EnumIter};
use serde::{Deserialize, Serialize};
use crate::entities::category_dish_map;
use crate::entities::prelude::{Categories, Category, Dish, Dishes};

#[derive(Clone, Debug, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "category_dish_map")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub category_id: String,
    #[sea_orm(primary_key, auto_increment = false)]
    pub dish_id: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::category::Entity",
        from = "Column::CategoryId",
        to = "super::category::Column::Id"
    )]
    Category,
    #[sea_orm(
        belongs_to = "super::dish::Entity",
        from = "Column::DishId",
        to = "super::dish::Column::Id"
    )]
    Dish,
}
impl Related<super::category::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Category.def()
    }
}
impl Related<super::dish::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Dish.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

pub struct CategoryToDish;
impl Linked for CategoryToDish {
    type FromEntity = Categories;
    type ToEntity = Dishes;

    fn link(&self) -> Vec<RelationDef> {
        vec![
            category_dish_map::Relation::Category.def().rev(),
            category_dish_map::Relation::Dish.def(),
        ]
    }
}

pub struct DishToCategory;
impl Linked for DishToCategory {
    type FromEntity = Dishes;
    type ToEntity = Categories;

    fn link(&self) -> Vec<RelationDef> {
        vec![
            category_dish_map::Relation::Dish.def().rev(),
            category_dish_map::Relation::Category.def(),
        ]
    }
}