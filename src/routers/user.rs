use salvo::oapi::extract::*;
use salvo::prelude::*;
use serde::{Deserialize};
use validator::Validate;

use crate::{JsonResult};
use crate::das::users::UserCurd;
use crate::dto::user::{CreateUserData, UserInfo};
use crate::utils::hash_password;

#[endpoint(tags("users"))]
pub async fn create_user(in_data: JsonBody<CreateUserData>) -> JsonResult<UserInfo> {
    let CreateUserData { username, password } = in_data.into_inner();
    let password = hash_password(&password)?;
    let id = UserCurd::insert_user(username.clone(), password).await?;
    Ok(Json(UserInfo {id, username}))
}

#[derive(Deserialize, Debug, Validate, ToSchema)]
struct UpdateInData {
    #[validate(length(min = 5, message = "username length must be greater than 5"))]
    username: String,
    #[validate(length(min = 6, message = "password length must be greater than 5"))]
    password: String,
}
// #[endpoint(tags("users"), parameters(("user_id", description = "user id")))]
// pub async fn update_user(
//     user_id: PathParam<String>,
//     idata: JsonBody<UpdateInData>,
// ) -> JsonResult<SafeUser> {
//     let user_id = user_id.into_inner();
//     let UpdateInData { username, password } = idata.into_inner();
//     let conn = db::pool();
// 
//     let Some(user) = Users::find_by_id(user_id).one(conn).await? else {
//         return Err(anyhow::anyhow!("User does not exist.").into());
//     };
//     let mut user: users::ActiveModel = user.into();
//     user.username = Set(username.to_owned());
//     user.password = Set(utils::hash_password(&password)?);
// 
//     let user: users::Model = user.update(conn).await?;
//     json_ok(SafeUser {
//         id: user.id,
//         username: user.username,
//     })
// }
// 
// #[endpoint(tags("users"))]
// pub async fn delete_user(user_id: PathParam<String>) -> EmptyResult {
//     let user_id = user_id.into_inner();
//     let conn = db::pool();
//     Users::delete_by_id(user_id).exec(conn).await?;
//     empty_ok()
// }
// 
// #[derive(Debug, Deserialize, Validate, Extractible, ToSchema)]
// #[salvo(extract(default_source(from = "query")))]
// pub struct UserListQuery {
//     pub username: Option<String>,
//     #[serde(default = "default_page")]
//     pub current_page: u64,
//     #[serde(default = "default_page_size")]
//     pub page_size: u64,
// }
// 
// fn default_page() -> u64 { 1 }
// fn default_page_size() -> u64 { 10 }
// 
// #[derive(Debug, Serialize, ToSchema)]
// pub struct UserListResponse {
//     pub data: Vec<SafeUser>,
//     pub total: u64,
//     pub current_page: u64,
//     pub page_size: u64,
// }
// 
// #[endpoint(tags("users"))]
// pub async fn list_users(query: &mut Request) -> JsonResult<UserListResponse> {
//     let query: UserListQuery = query.extract().await?;
//     let conn = db::pool();
//     
//     let mut select = Users::find();
//     
//     // Apply username filter if provided
//     if let Some(username) = query.username.as_ref() {
//         select = select.filter(users::Column::Username.contains(username));
//     }
//     
//     // Get total count
//     let total = select.clone().count(conn).await?;
//     
//     // Apply pagination
//     let users = select
//         .offset(((query.current_page - 1) * query.page_size) as u64)
//         .limit(query.page_size)
//         .all(conn)
//         .await?
//         .into_iter()
//         .map(|user| SafeUser {
//             id: user.id,
//             username: user.username,
//         })
//         .collect::<Vec<_>>();
//     
//     json_ok(UserListResponse {
//         data: users,
//         total,
//         current_page: query.current_page,
//         page_size: query.page_size,
//     })
// }