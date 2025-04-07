use salvo::prelude::*;
use crate::config::get_config;
use crate::hoops::jwt::auth_hoop;
use crate::hoops::jwt;

mod auth;
mod user;
mod menu;

pub fn root() -> Router {
    let router = Router::new()
        .push(
            Router::with_path("validate_token")
                .get(jwt::validate_token)
        )
        .push(
            Router::with_path("create_user")
                .post(user::create_user)
        )
        .push(
            Router::with_path("login")
                .post(auth::post_login)
        )
        .push(
            Router::new()
                .hoop(auth_hoop(&get_config().jwt))
                .push(
                    Router::with_path("create")
                        .push(
                            Router::with_path("category")
                                .post(menu::create_category)
                        )
                        .push(
                            Router::with_path("dish")
                                .post(menu::create_dish)
                        )
                )
                .push(
                    Router::with_path("delete")
                        .push(
                            Router::with_path("category/{id}")
                                .delete(menu::delete_category)
                        )
                        .push(
                            Router::with_path("dish/{id}")
                                .delete(menu::delete_dish)
                        )
                )
                .push(
                    Router::with_path("get")
                        .push(
                            Router::with_path("menu")
                                .get(menu::get_menu)
                        )
                        .push(
                            Router::with_path("all_categories")
                                .get(menu::get_all_categories)
                        )
                        .push(
                            Router::with_path("all_dishes")
                                .get(menu::get_all_dishes)
                        )
                        .push(
                            Router::with_path("dish_by_category/{id}")
                                .get(menu::get_dishes_by_category)
                        )
                )
        );
    router
}
