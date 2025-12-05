use std::{env, fs};

fn main() {
    // android 타겟일 때만 처리 (x86_64-linux-android 등)
    let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap_or_default();
    if target_os != "android" {
        return;
    }

    // .mobile.env 읽어서 MOBILE_DATABASE_URL 찾기
    let env_file = ".mobile.env";
    let content = fs::read_to_string(env_file).expect(".mobile.env 파일을 읽을 수 없음");

    for line in content.lines() {
        // 주석, 빈줄 스킵
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        if let Some(value) = line.strip_prefix("MOBILE_DATABASE_URL=") {
            // 컴파일 타임 env로 주입
            println!("cargo:rustc-env=MOBILE_DATABASE_URL={value}");
            return;
        }
    }

    panic!(".mobile.env에 MOBILE_DATABASE_URL=... 이 없음");
}
