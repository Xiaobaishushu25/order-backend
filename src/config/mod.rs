mod db;

use std::{env, fs, io, panic};
use std::fs::{File, OpenOptions};
use std::io::Write;
use std::path::PathBuf;
use std::sync::{LazyLock, OnceLock};
use log::{error, info};
use rand::distr::Alphanumeric;
use rand::Rng;
use serde::{Deserialize, Serialize};
use time::macros::format_description;
use time::UtcOffset;
use tracing_appender::non_blocking::WorkerGuard;
use tracing_appender::rolling::{RollingFileAppender, Rotation};
use tracing_subscriber::{fmt, EnvFilter, Layer, Registry};
use tracing_subscriber::fmt::time::OffsetTime;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use crate::error::AppResult;

pub static CURRENT_DIR: LazyLock<PathBuf> = LazyLock::new(|| {
    let current_dir = env::current_dir().expect("无法获取当前目录");
    current_dir
});

pub static CONFIG: OnceLock<ServerConfig> = OnceLock::new();

fn check_config_file(path: &PathBuf, current_dir: &PathBuf) -> AppResult<ServerConfig> {
    let mut config_file: File = if path.exists() {
        info!("配置存在");
        if let Ok(config) = toml::from_str::<ServerConfig>(&fs::read_to_string(path)?) {
            return Ok(config); //如果正确解析配置文件，直接返回
        } else {
            error!("配置文件格式错误，将重新创建配置文件。");
            //清除配置文件内容
            // 打开文件并清空内容
            OpenOptions::new()
                .write(true) // 以写入模式打开文件
                .truncate(true) // 清空文件内容
                .open(path)?
        }
    } else {
        info!("配置不存在,创建配置。");
        // fs::create_dir_all(format!("{}/data", current_dir))?;
        fs::create_dir_all(current_dir.join("data"))?;
        File::create(path)?
    };
    //如果上面正确读取配置文件就已经返回了，到这里说明配置文件没有内容，需要初始化默认配置
    // let config = Config::init_default();
    let config = ServerConfig::default();
    let config_string = toml::to_string(&config).map_err(|e| {anyhow::anyhow!("ServerConfig序列化错误")})?;
    config_file.write_all(config_string.as_bytes())?;
    Ok(config)
}

pub fn get() -> &'static ServerConfig {
    CONFIG.get().expect("config should be set")
}
pub fn init_config() {
    let config = check_config_file(&CURRENT_DIR.join("data").join("config.toml"), &CURRENT_DIR).expect("无法加载配置文件");
    CONFIG.set(config.clone()).expect("无法设置config");
}

#[derive(Deserialize, Serialize, Clone, Debug,Default)]
pub struct ServerConfig {
    #[serde(default = "default_listen_addr")]
    pub listen_addr: String,
    pub jwt: JwtConfig,
}
#[derive(Deserialize, Serialize, Clone, Debug, Default)]
pub struct JwtConfig {
    #[serde(default = "generate_secret(32)")]
    pub secret: String,
    pub expiry: i64,
}

fn default_listen_addr() -> String {
    "127.0.0.1:8008".into()
}
fn generate_secret(length: usize) -> String {
    let mut rng = rand::rng();
    let secret: String = (0..length)
        .map(|_| rng.sample(Alphanumeric) as char)
        .collect();
    secret
}

/// 初始化日志
pub fn init_logger() -> WorkerGuard {
    // 配置文件日志
    // let log_path = format!("{}/data/log", CURRENT_DIR.clone());
    let log_path = CURRENT_DIR.join("data").join("log");
    fs::create_dir_all(&log_path).expect("无法创建日志目录");

    let local_time = OffsetTime::new(
        UtcOffset::from_hms(8, 0, 0).unwrap(),
        format_description!("[year]-[month]-[day] [hour]:[minute]:[second].[subsecond digits:3]"),
    );

    let file_appender = RollingFileAppender::builder()
        .rotation(Rotation::DAILY)
        .filename_prefix("order") //意味着生成的日志文件名会以 "litManagePro" 开头。
        .filename_suffix("log") //生成的日志文件名会以 .log 结尾
        .build(log_path)
        .expect("无法初始化滚动文件追加器");

    let (non_blocking_file, worker_guard) = tracing_appender::non_blocking(file_appender);
    let file_layer = fmt::layer()
        .with_writer(non_blocking_file)
        .with_ansi(false) //表示不使用 ANSI 转义码。这通常用于文件日志，因为文件通常不支持 ANSI 转义码（如颜色、样式等）。
        .with_line_number(true) //表示在日志中包含行号。这有助于调试时快速定位日志的来源。
        .with_target(true) //表示在日志中包含目标。目标通常是一个字符串，用于标识日志的来源，例如模块名或函数名。
        // .with_thread_ids(true)//表示在日志中包含线程 ID。这有助于区分不同线程的日志，特别是在多线程环境中。
        .with_level(true) //表示在日志中包含日志级别（如 INFO、ERROR 等）。这有助于快速识别日志的严重性。
        .with_thread_names(true)
        .with_timer(local_time.clone())
        .with_filter(EnvFilter::new("info"));

    // 配置控制台日志
    let console_layer = fmt::layer()
        .with_writer(io::stdout)
        .with_ansi(true)
        .with_line_number(true)
        .with_target(true)
        // .with_thread_ids(true)
        .with_level(true)
        // .with_thread_names(true)
        .with_timer(local_time)
        .with_filter(EnvFilter::new(
            "info,tao::platform_impl::platform::event_loop::runner=error",
        ));
    // .with_filter(EnvFilter::new("info")); // 控制台显示 info 级别及以上的日志

    // 配置日志订阅器
    Registry::default()
        .with(console_layer)
        .with(file_layer)
        .with(EnvFilter::new("info"))
        .init();

    // tracing::subscriber::set_global_default(subscriber)
    //     .expect("设置日志订阅器失败");
    panic::set_hook(Box::new(|info| {
        if let Some(location) = info.location() {
            // 打印 panic 信息和发生 panic 的位置
            error!(
                "Panic occurred at {}:{}:{}",
                location.file(),
                location.line(),
                location.column()
            );
        }
        // 处理panic payload，检查是否为某个具体的错误类型
        if let Some(payload) = info.payload().downcast_ref::<String>() {
            // 如果payload是字符串类型，直接打印
            error!("Panic message: {}", payload);
        } else if let Some(payload) = info.payload().downcast_ref::<&str>() {
            // 如果是&str，直接打印
            error!("Panic message: {}", payload);
        } else {
            // 其他情况，打印更通用的信息
            error!("Panic occurred with unknown payload: {:?}", info.payload());
        }
    }));
    worker_guard
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_init_config() {
        init_config();
        println!("{:?}", CONFIG.get().unwrap());
    }
}