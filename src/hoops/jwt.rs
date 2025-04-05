use anyhow::Result;
use jsonwebtoken::{decode, Algorithm, DecodingKey, EncodingKey, Validation};
use log::info;
use salvo::jwt_auth::{ConstDecoder, CookieFinder, HeaderFinder, QueryFinder};
use salvo::prelude::*;
use serde::{Deserialize, Serialize};
use time::{Duration, OffsetDateTime};

use crate::config::{self, get_config, JwtConfig};

#[derive(Debug, Serialize, Deserialize)]
pub struct JwtClaims {
    uid: String,
    exp: i64,
}

/// 这段代码的功能是创建一个JWT认证中间件 `JwtAuth`，具体逻辑如下：
/// 1. 使用 `ConstDecoder` 从配置中的密钥 `secret` 创建解码器。
/// 2. 配置多个查找器（`HeaderFinder`, `QueryFinder`, `CookieFinder`），用于从请求头、查询参数和Cookie中查找JWT令牌。
/// 3. 设置 `force_passed` 为 `false`，表示即使没有找到令牌也不强制抛出错误。
pub fn auth_hoop(config: &JwtConfig) -> JwtAuth<JwtClaims, ConstDecoder> {
    info!("JwtAuth init");
    JwtAuth::new(ConstDecoder::from_secret(
        config.secret.to_owned().as_bytes(),
    ))
    .finders(vec![
        Box::new(HeaderFinder::new()),
        Box::new(QueryFinder::new("token")),
        Box::new(CookieFinder::new("jwt_token")),
    ])
    .force_passed(false)
}

/// 这段代码的功能是生成一个带有过期时间的JWT令牌，具体逻辑如下：  
/// 1. 获取当前UTC时间并加上配置的过期时间，计算出令牌的有效期。  
/// 2. 构造`JwtClaims`结构体，包含用户ID (`uid`) 和过期时间戳 (`exp`)。  
/// 3. 使用`jsonwebtoken`库对`JwtClaims`进行编码，生成签名后的JWT字符串。  
/// 4. 返回生成的JWT字符串和过期时间戳。
pub fn get_token(uid: impl Into<String>) -> Result<(String, i64)> {
    let exp = OffsetDateTime::now_utc() + Duration::seconds(get_config().jwt.expiry);
    let claim = JwtClaims {
        uid: uid.into(),
        exp: exp.unix_timestamp(),
    };
    let token: String = jsonwebtoken::encode(
        &jsonwebtoken::Header::default(),
        &claim,
        &EncodingKey::from_secret(get_config().jwt.secret.as_bytes()),
    )?;
    Ok((token, exp.unix_timestamp()))
}

pub fn decode_token(token: &str) -> bool {
    let validation = Validation::new(Algorithm::HS256);
    decode::<JwtClaims>(
        token,
        &DecodingKey::from_secret(get_config().jwt.secret.as_bytes()),
        &validation,
    )
    .is_ok()
}
