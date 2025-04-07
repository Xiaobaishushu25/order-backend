use salvo::prelude::ToSchema;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize, ToSchema, Debug)]
pub struct UserInfo{
    pub id: String,
    pub username: String,
}

#[derive(Deserialize, Debug, Validate, ToSchema, Default)]
pub struct CreateUserData {
    #[validate(length(min = 5, message = "username length must be greater than 5"))]
    pub username: String,
    #[validate(length(min = 6, message = "password length must be greater than 5"))]
    pub password: String,
}