use sea_orm::{PaginatorTrait, QueryOrder};
use sea_orm::{EntityTrait, IntoActiveModel};
use ulid::Ulid;
use crate::config::db::get_db_coon;
use crate::entities::dish::{Column, Status};
use crate::entities::prelude::{Dish, Dishes};
use crate::error::AppResult;
use crate::utils::get_now_time;

pub struct DishCurd;
impl DishCurd {
    pub async fn insert(name: String, price: f64, picture: String) -> AppResult<String> {
        let db = get_db_coon();
        let uuid = Ulid::new();
        let index = Dishes::find().count(db).await?+1;
        let dish = Dish {
            id: uuid.to_string(),
            index: index as i32,
            name,
            price,
            picture,
            status: Status::Normal,
            created_at: get_now_time(),
        };
        Dishes::insert(
            dish.into_active_model()
        ).exec(db).await?;
        Ok(uuid.to_string())
    }
    pub async fn query_all() -> AppResult<Vec<Dish>> {
        let db = get_db_coon();
        let dishes = Dishes::find()
            .order_by_asc(Column::Index)
            .all(db)
            .await?;
        Ok(dishes)
    }
    pub async fn delete_by_id(id: String) -> AppResult<()> {
        let db = get_db_coon();
        Dishes::delete_by_id(id).exec(db).await?;
        Ok(())
    }
}
#[cfg(test)]
mod tests {
    use crate::config::db::init_db_coon;
    use crate::das::dish::DishCurd;

    #[tokio::test]
    async fn test_create_dish() {
        init_db_coon().await;
        DishCurd::insert("test".to_string(), 100.0, "test".to_string()).await.unwrap();
    }
}