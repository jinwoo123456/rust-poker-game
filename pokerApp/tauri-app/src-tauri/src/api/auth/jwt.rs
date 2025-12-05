use anyhow::Result;
use dotenvy::dotenv;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::env;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Serialize, Deserialize)]
struct Claims {
    sub: String,
    exp: usize,
}

pub fn get_secret_key() -> Result<String> {
    // 안드로이드 빌드
    #[cfg(target_os = "android")]
    {
        // build.rs에서 주입한 컴파일타임 env 사용
        Ok(env!("MOBILE_SECRET_KEY").to_string())
    }

    // 데스크탑(Windows, Linux, macOS) 빌드
    #[cfg(not(target_os = "android"))]
    {
        dotenv().ok(); // .env 로드
        Ok(env::var("SECRET_KEY")?)
    }
}

pub fn create_jwt(username: &str) -> Result<String> {
    // .env 로드 (이미 로드돼 있으면 no-op)
    dotenv().ok();
    let secret = get_secret_key()?;

    let expiration = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs() + 3600; // 1시간 유효
    let claims = Claims {
        sub: username.to_string(),
        exp: expiration as usize,
    };

    let key = EncodingKey::from_secret(secret.as_bytes());
    let token = encode(&Header::default(), &claims, &key)?;
    Ok(token)
}

pub fn decode_jwt(token: &str) -> Result<String> {
    dotenv().ok();
    let secret = env::var("SECRET_KEY")?;
    let key = DecodingKey::from_secret(secret.as_bytes());
    let data = decode::<Claims>(token, &key, &Validation::default())?;
    Ok(data.claims.sub)
}
