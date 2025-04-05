pub mod db;
pub mod log_config;

use std::{env, fs, panic};
use std::fs::{File, OpenOptions};
use std::io::Write;
use std::path::PathBuf;
use std::sync::{LazyLock, OnceLock};
use log::info;
use log::error;
use rand::distr::Alphanumeric;
use rand::Rng;
use serde::{Deserialize, Serialize};
use tracing_subscriber::{Layer, Registry};
use crate::config::log_config::LogConfig;
use crate::error::AppResult;
use crate::utils::check_file;

pub static CURRENT_DIR: LazyLock<PathBuf> = LazyLock::new(|| {
    let current_dir = env::current_dir().expect("无法获取当前目录");
    current_dir
});

pub static CONFIG: OnceLock<ServerConfig> = OnceLock::new();

pub fn get_config() -> &'static ServerConfig {
    CONFIG.get().expect("config should be set")
}
/// 初始化配置文件,并返回配置
/// 只应该在初始化时调用一次，后续需要配置时请使用[get_config()]
pub fn load_config()->&'static ServerConfig{
    let config = check_config_file(&CURRENT_DIR.join("data").join("config.toml"), &CURRENT_DIR).expect("无法加载配置文件");
    CONFIG.set(config.clone()).expect("无法设置config");
    CONFIG.get().expect("config should be set")
}

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

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct ServerConfig {
    pub listen_addr: String,
    pub jwt: JwtConfig,
    pub log: LogConfig,
}
impl Default for ServerConfig {
    fn default() -> Self {
        ServerConfig {
            listen_addr: "127.0.0.1:8008".into(),
            jwt: JwtConfig::default(),
            log: LogConfig::default(),
        }
    }
}
#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct JwtConfig {
    pub secret: String,
    pub expiry: i64,
}

impl Default for JwtConfig {
    fn default() -> Self {
        JwtConfig {
            secret: generate_secret(32),
            expiry: 3600,
        }
    }
}
fn generate_secret(length: usize) -> String {
    let mut rng = rand::rng();
    let secret: String = (0..length)
        .map(|_| rng.sample(Alphanumeric) as char)
        .collect();
    secret
}


#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_init_config() {
        load_config();
        println!("{:?}", get_config());
    }
}