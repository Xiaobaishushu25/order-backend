use anyhow::anyhow;
use sea_orm::{EntityTrait, IntoActiveModel, ModelTrait, QueryOrder};
use ulid::Ulid;
use crate::config::db::get_db_coon;
use crate::entities::category::Column;
use crate::entities::category_dish_map::CategoryToDish;
use crate::entities::prelude::{Categories, Category, Dish};
use crate::error::AppResult;

pub struct CategoryCurd;
impl CategoryCurd {
    /// 插入分类, 返回分类id
    pub async fn insert(name: String) -> AppResult<String> {
        let db = get_db_coon();
        let uuid = Ulid::new();
        let index = Categories::find()
            .order_by_desc(Column::Index)
            .one(db)
            .await?
            .map(|c| c.index + 1)
            .unwrap_or(0);
        let category = Category {
            id: uuid.to_string(),
            index,
            name,
        };
        Categories::insert(
            category
            .into_active_model(),
        )
        .exec(db)
        .await?;
        Ok(uuid.to_string())
    }
    pub async fn delete_by_id(id: String) -> AppResult<()> {
        let db = get_db_coon();
        Categories::delete_by_id(id).exec(db).await?;
        Ok(())
    }
    pub async fn query_all() -> AppResult<Vec<Category>> {
        let db = get_db_coon();
        Ok(Categories::find()
            .order_by_asc(Column::Index)
            .all(db)
            .await?)
    }
    /// 查询分类关联的菜品
    pub async fn query_related_dishes(id: String) -> AppResult<Vec<Dish>> {
        let db = get_db_coon();
        let category = Categories::find_by_id(id.clone()).one(db).await?.ok_or_else(|| anyhow!("id为{}的种类不存在",id))?;
        Ok(category.find_linked(CategoryToDish).all(db).await?)
    }
}