use log::info;
use salvo::oapi::extract::*;
use salvo::prelude::*;
use serde::{Deserialize, Serialize};

use crate::das::users::UserCurd;
use crate::entities::users::Model;
use crate::hoops::jwt;
use crate::{JsonResult, utils};

#[derive(Deserialize, ToSchema, Default, Debug)]
pub struct LoginInData {
    pub username: String,
    pub password: String,
}
#[derive(Serialize, ToSchema, Default, Debug)]
pub struct LoginOutData {
    pub id: String,
    pub username: String,
    pub token: String,
    pub exp: i64,
}
#[endpoint(tags("auth"))]
pub async fn post_login(
    in_data: JsonBody<LoginInData>,
    res: &mut Response,
) -> JsonResult<LoginOutData> {
    let idata = in_data.into_inner();
    info!("login:{:?}",idata);
    let Some(Model {
        id,
        username,
        password,
    }) = UserCurd::query_by_username(idata.username).await?
    else {
        return Err(StatusError::unauthorized()
            .brief("User does not exist.")
            .detail("User does not exist.")
            .into());
    };
    if utils::verify_password(&idata.password, &password).is_err() {
        return Err(StatusError::unauthorized()
            .brief("password is incorrect.")
            .into());
    }

    let (token, exp) = jwt::get_token(&id)?;
    let out_data = LoginOutData {
        id,
        username,
        token,
        exp,
    };
    // let cookie = Cookie::build(("jwt_token", out_data.token.clone()))
    //     .path("/")
    //     .http_only(true)
    //     .build();
    // res.add_cookie(cookie);
    Ok(Json(out_data))
}
