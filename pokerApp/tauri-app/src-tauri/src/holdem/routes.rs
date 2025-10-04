use axum::{
    routing::{get, post},
    Router,
};
use crate::AppState;
use super::handlers;




pub fn router() -> Router<AppState> {
    // NOTE: 이전에는 trait 메서드를 직접 참조해 컴파일 오류가 났습니다.
    //       handlers 모듈에 자유 함수가 추가되었으므로 해당 함수 포인터를 그대로 넘깁니다.
    Router::new()
        .route("/api/holdem/start", post(handlers::start_game))
        // .route("/api/holdem/mycard", post(handlers::get_my_cards))
        // .route("/api/holdem/flop", get(handlers::flop))
        // .route("/api/holdem/turn", get(handlers::turn))
        // .route("/api/holdem/rivers", get(handlers::rivers))
}
