use axum::{routing::get, Router};
use dotenvy::dotenv;
use sea_orm::{ConnectOptions, Database, DatabaseConnection};
use std::env;
use std::time::Duration;

/// 모바일과 데스크탑에서 db 연결을 따로 설정.
/// 모바일은 build.rs에서 설정한 상수로 db 연결 처리
/// 데스크탑은 .env 파일에서설정한 상수로 db 연결 처리
#[cfg(mobile)]
fn db_url() -> String {
    // build.rs에서 주입한 값 사용
    option_env!("MOBILE_DATABASE_URL")
        .expect("MOBILE_DATABASE_URL not set at build time (check src-tauri/.mobile.env)")
        .to_string()
}

#[cfg(not(mobile))]
fn db_url() -> String {
    // 데스크탑은 .env 사용
    dotenvy::dotenv().ok();
    dotenvy::var("DATABASE_URL").expect("DATABASE_URL 없음 (.env 확인)")
}

pub async fn db_connect() -> DatabaseConnection {
    let url = db_url();

    let mut options = ConnectOptions::new(url);

    options
        .max_connections(10)
        .min_connections(1)
        .sqlx_logging(false)
        .connect_timeout(Duration::from_secs(5))
        .acquire_timeout(Duration::from_secs(5));

    Database::connect(options).await.expect("DB 연결 실패")
}
