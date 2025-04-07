use log::error;
use salvo::oapi::ToSchema;
use sea_orm::{EntityTrait, ModelTrait};
use serde::{Deserialize, Serialize};
use crate::config::db::get_db_coon;
use crate::entities::category_dish_map::CategoryToDish;
use crate::entities::prelude::{Categories, Category, Dish};

pub mod user;
pub mod menu;