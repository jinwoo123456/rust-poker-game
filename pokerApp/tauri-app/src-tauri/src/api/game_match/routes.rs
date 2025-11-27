use super::handlers;
use crate::AppState;
use axum::{
    routing::{get, post},
    Router,
};

pub fn router() -> Router<AppState> {
    // NOTE: 이전에는 trait 메서드를 직접 참조해 컴파일 오류가 났습니다.
    //       handlers 모듈에 자유 함수가 추가되었으므로 해당 함수 포인터를 그대로 넘깁니다.
    Router::new()
        .route("/api/match/solocreate", post(handlers::create_solo_match_player))
        .route("/api/match/create", post(handlers::create_match_player))
        .route("/api/match/show", get(handlers::show_match_player_list))
}
