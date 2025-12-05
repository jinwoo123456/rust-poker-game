// src-tauri/build.rs

use std::{env, fs, path::Path};

fn main() {
    // (모바일 프로젝트면) tauri 모바일 초기화가 필요할 수도 있음
    // tauri_build::mobile::init();   // 만약 이미 쓰고 있다면 그대로 둬

    // 1. .mobile.env 로부터 환경변수 로드
    load_mobile_env();

    // 2. 기존 Tauri 빌드 작업 유지
    tauri_build::build();
}

fn load_mobile_env() {
    println!("cargo:rerun-if-changed=.mobile.env");

    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let env_path = Path::new(&manifest_dir).join(".mobile.env");

    let Ok(content) = fs::read_to_string(env_path) else {
        eprintln!(".mobile.env not found, skipping mobile env setup");
        return;
    };

    for line in content.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        if let Some((key, value)) = line.split_once('=') {
            let value = value.trim();
            // 여기서 필요한 키들만 골라서 주입
            if key == "MOBILE_DATABASE_URL" || key == "MOBILE_SECRET_KEY" {
                println!("cargo:rustc-env={}={}", key, value);
            }
        }
    }
}
