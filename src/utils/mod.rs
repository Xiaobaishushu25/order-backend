use std::fs;
use std::fs::File;
use std::path::PathBuf;
use argon2::password_hash::SaltString;
use argon2::{Argon2, PasswordHash};
use argon2::password_hash::rand_core::OsRng;
use crate::error::AppResult;

/// 检查文件是否存在，不存在则创建文件
pub fn check_file(path: &PathBuf) ->AppResult<bool>{
    if path.exists() {
        Ok(true)
    } else {
        let _ = fs::create_dir_all(path)?;
        let _ = File::create(path)?;
        Ok(false)
    }
}
pub fn verify_password(password: &str, password_hash: &str) -> AppResult<()> {
    let hash = PasswordHash::new(&password_hash)
        .map_err(|e| anyhow::anyhow!("invalid password hash: {}", e))?;
    let result = hash.verify_password(&[&Argon2::default()], password);
    match result {
        Ok(_) => Ok(()),
        Err(_) => Err(anyhow::anyhow!("invalid password"))?,
    }
}
pub fn hash_password(password: &str) -> AppResult<String> {
    let salt = SaltString::generate(&mut OsRng);
    Ok(PasswordHash::generate(Argon2::default(), password, &salt)
        .map_err(|e| anyhow::anyhow!("failed to generate password hash: {}", e))?
        .to_string())
}
