use salvo::cors::Cors;
use salvo::prelude::*;
use crate::config::get_config;
use crate::hoops::jwt::auth_hoop;
use salvo::http::Method;

mod auth;
mod user;


pub fn root() -> Router {
    let router = Router::new()
        // .hoop(Logger::new())
        // .hoop(cors)
        .push(
            Router::new()
                .hoop(auth_hoop(&get_config().jwt))
        )
        .push(
            Router::with_path("create_user")
                .post(user::create_user)
        )
        .push(
            Router::with_path("login")
                .post(auth::post_login)
        );
    router
}
