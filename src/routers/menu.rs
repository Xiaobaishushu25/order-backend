use salvo::Writer;
use log::info;
use salvo::handler;
use salvo::oapi::extract::{JsonBody, PathParam};
use salvo::prelude::Json;
use crate::das::category::CategoryCurd;
use crate::das::category_dish_map::CategoryDishMapCurd;
use crate::das::dish::DishCurd;
use crate::dto::menu::{query_menu, CategoryWithDishes, CreateCategoryData, CreateDishData};
use crate::entities::prelude::{Category, Dish};
use crate::JsonResult;

#[handler]
pub async fn create_category(data:JsonBody<CreateCategoryData>)->JsonResult<String>{
    let data = data.into_inner();
    let id = CategoryCurd::insert(data.name).await?;
    for dish_id in data.dish_ids{
        CategoryDishMapCurd::insert(id.clone(), dish_id).await?;
    }
    Ok(Json(id))
}
#[handler]
pub async fn create_dish(data:JsonBody<CreateDishData>)->JsonResult<String>{
    let data = data.into_inner();
    let id = DishCurd::insert(data.name, data.price, data.picture).await?;
    for category_id in data.category_ids{
        CategoryDishMapCurd::insert(category_id, id.clone()).await?;
    }
    Ok(Json(id))
}
#[handler]
pub async fn delete_category(id:PathParam<String>)->JsonResult<()>{
    let id = id.into_inner();
    CategoryDishMapCurd::delete_by_category_id(id.clone()).await?;
    CategoryCurd::delete_by_id(id).await?;
    Ok(Json(()))
}
#[handler]
pub async fn delete_dish(id:PathParam<String>)->JsonResult<()>{
    let id = id.into_inner();
    CategoryDishMapCurd::delete_by_dish_id(id.clone()).await?;
    DishCurd::delete_by_id(id).await?;
    Ok(Json(()))
}
#[handler]
pub async fn get_menu()->Json<Vec<CategoryWithDishes>>{
    info!("get menu");
    Json(query_menu().await)
}
#[handler]
pub async fn get_all_categories()->JsonResult<Vec<Category>>{
    let models = CategoryCurd::query_all().await?;
    Ok(Json(models))
}
#[handler]
pub async fn get_all_dishes()->JsonResult<Vec<Dish>>{
    let models = DishCurd::query_all().await?;
    Ok(Json(models))
}
#[handler]
pub async fn get_dishes_by_category(id:PathParam<String>) ->JsonResult<Vec<Dish>>{
    let id = id.into_inner();
    let models = CategoryCurd::query_related_dishes(id).await?;
    Ok(Json(models))
}