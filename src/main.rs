use salvo::prelude::{Json, TcpListener, ToSchema};
use salvo::{Listener, Server, Service};
use salvo::cors::Cors;
use salvo::http::Method;
use salvo::logging::Logger;
use serde::Serialize;
use tracing_appender::non_blocking::WorkerGuard;
use crate::config::{get_config, load_config};
use crate::config::db::init_db_coon;
use crate::config::log_config::init_logger;
use crate::error::AppError;

mod error;
mod config;
mod entities;
mod utils;
mod routers;
mod hoops;
mod das;
mod dto;

pub type JsonResult<T> = Result<Json<T>, AppError>;
pub type EmptyResult = Result<Json<Empty>, AppError>;

#[derive(Serialize, ToSchema, Clone, Copy, Debug)]
pub struct Empty {}
pub fn empty_ok() -> JsonResult<Empty> {
    Ok(Json(Empty {}))
}

#[tokio::main]
async fn main() {
    let (_log_guard) = init_all().await;
    let cors = Cors::new()
        .allow_origin("http://localhost:5173")
        .allow_methods(vec![Method::GET, Method::POST, Method::DELETE, Method::OPTIONS])
        .allow_headers(vec!["Content-Type","Authorization"]) // 添加允许的请求头
        .into_handler();
    let service = Service::new(routers::root()).hoop(Logger::new()).hoop(cors);
    println!("🔄 在以下位置监听 {}", get_config().listen_addr);
    let acceptor = TcpListener::new(&get_config().listen_addr).bind().await;
    let server = Server::new(acceptor);
    server.serve(service).await;
}
async fn init_all()->(WorkerGuard){
    let config = load_config();
    let log_guard = init_logger(&config.log);
    init_db_coon().await;
    (log_guard)
}
