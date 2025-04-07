use sea_orm::{ColumnTrait, QueryFilter};
use sea_orm::{EntityTrait, IntoActiveModel};
use crate::config::db::get_db_coon;
use crate::entities::category_dish_map::Column;
use crate::entities::prelude::{CategoryDishMap, CategoryDishMaps};
use crate::error::AppResult;

pub struct CategoryDishMapCurd;
impl CategoryDishMapCurd {
    pub async fn insert(category_id: String, dish_id: String) -> AppResult<()> {
        let db = get_db_coon();
        let category_dish_map = CategoryDishMap {
            category_id,
            dish_id,
        };
        CategoryDishMaps::insert(
            category_dish_map
                .into_active_model(),
        )
        .exec(db)
        .await?;
        Ok(())
    }
    pub async fn delete_by_category_id(category_id: String) -> AppResult<()> {
        let db = get_db_coon();
        CategoryDishMaps::delete_many()
            .filter(Column::CategoryId.eq(category_id))
            .exec(db).await?;
        Ok(())
    }
    pub async fn delete_by_dish_id(dish_id: String) -> AppResult<()> {
        let db = get_db_coon();
        CategoryDishMaps::delete_many()
            .filter(Column::DishId.eq(dish_id))
            .exec(db).await?;
        Ok(())
    }
}