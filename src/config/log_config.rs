use std::{fs, io, panic};
use log::{error, info};
use serde::{Deserialize, Serialize};
use time::macros::format_description;
use time::UtcOffset;
use tracing_appender::non_blocking::WorkerGuard;
use tracing_appender::rolling::{RollingFileAppender, Rotation};
use tracing_subscriber::{fmt, EnvFilter, Layer, Registry};
use tracing_subscriber::fmt::time::OffsetTime;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use crate::config::CURRENT_DIR;

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct LogConfig{
    pub file_name:String,
    // 记录到文件的日志级别 info debug error
    pub level:String,
    // 滚动日志 daily hourly never
    pub rolling:String,
}
impl Default for LogConfig{
    fn default() -> Self {
        LogConfig{
            file_name: "order.log".into(),
            level: "info".into(),
            rolling: "daily".into(),
        }
    }
}
/// 初始化日志
pub fn init_logger(log_config: &LogConfig) -> WorkerGuard {
    // 配置文件日志
    // let log_path = format!("{}/data/log", CURRENT_DIR.clone());
    let log_path = CURRENT_DIR.join("data").join("log");
    fs::create_dir_all(&log_path).expect("无法创建日志目录");

    let local_time = OffsetTime::new(
        UtcOffset::from_hms(8, 0, 0).unwrap(),
        format_description!("[year]-[month]-[day] [hour]:[minute]:[second].[subsecond digits:3]"),
    );
    let rolling = match log_config.rolling.as_str() { 
        "daily" => Rotation::DAILY,
        "hourly" => Rotation::HOURLY,
        "never" => Rotation::NEVER,
        _ => Rotation::DAILY,
    };
    let file_appender = RollingFileAppender::builder()
        .rotation(rolling)
        .filename_prefix(log_config.file_name.clone()) //意味着生成的日志文件名会以 "litManagePro" 开头。
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
        .with_filter(EnvFilter::new(log_config.level.to_string()));

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