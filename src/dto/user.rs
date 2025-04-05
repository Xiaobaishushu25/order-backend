use salvo::prelude::ToSchema;
use serde::Serialize;

#[derive(Serialize, ToSchema, Debug)]
pub struct UserInfo{
    pub id: String,
    pub username: String,
}