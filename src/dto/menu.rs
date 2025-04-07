use log::error;
use salvo::oapi::ToSchema;
use sea_orm::{EntityTrait, ModelTrait};
use serde::{Deserialize, Serialize};
use crate::config::db::get_db_coon;
use crate::entities::category_dish_map::CategoryToDish;
use crate::entities::prelude::{Categories, Category, Dish};

#[derive(Deserialize, Debug, ToSchema, Default)]
pub struct CreateCategoryData {
    pub name: String,
    pub dish_ids: Vec<String>,
}

#[derive(Deserialize, Debug, ToSchema, Default)]
pub struct CreateDishData {
    pub name: String,
    pub price:f64,
    pub picture:String,
    pub category_ids: Vec<String>,
}

#[derive(Debug,Deserialize,Serialize)]
pub struct CategoryWithDishes{
    pub category: Category,
    pub dish: Vec<Dish>,
}

pub async fn query_menu()->Vec<CategoryWithDishes>{
    let db = get_db_coon();
    let categories = Categories::find()
        .all(db)
        .await
        .unwrap();
    let mut result = Vec::new();
    for category in categories{
        match category.find_linked(CategoryToDish).all(db).await{
            Ok(dishes) => {
                result.push(CategoryWithDishes{
                    category,
                    dish: dishes,
                });
            },
            Err(e) => {
                error!("query menu error: {}", e)
            }
        }
    };
    result
}