//! FIXME 안내: 현재 컴파일 에러의 핵심 원인을 단계별로 정리한 주석입니다.
//! 1) trait 선언부의 시그니처는 `async fn foo(&self, State<AppState>, Json<Dto>) -> impl IntoResponse;`
//!    처럼 구조 분해 없이 타입만 표기해야 하며, 구현부에서는 같은 형태의 시그니처를 사용해야 합니다.
//! 2) 반환 타입은 `impl IntoResponse`로 선언되어 있으므로, 구현부에서도 `(StatusCode, Json(body))`처럼
//!    IntoResponse를 만족하는 값을 반환해야 합니다. 현재는 반환 타입이 누락되어 경고가 납니다.
//! 3) DTO 이름(`MatchPlayerList`)과 실제 정의가 맞는지 확인하세요. 없으면 dto 모듈에 정의를 추가해야 합니다.
//! 4) `AppState`를 사용하려면 `use crate::AppState;`가 필요합니다. 아래에서는 이미 임포트되어 있다고 가정합니다.
//! 5) 함수 네이밍은 러스트 컨벤션에 따라 snake_case(`create_match_player`)로 두면 경고를 피할 수 있습니다.
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::{extract::State, Json};
use chrono::{DateTime, FixedOffset, Utc};
use sea_orm::{ConnectionTrait, DatabaseBackend, DatabaseConnection, QueryResult, Statement};
use serde_json::json;

use super::dto;
use crate::{
    api::holdem::handlers::{start_holdem_game_for_players, GameCreationResult},
    AppState,
};


// pub trait MatchTrait { ... }
// pub trait MatchPlayerListTrait { ... }
// impl MatchTrait for dto::MatchPlayer { ... }
// impl MatchPlayerListTrait for dto::MatchPlayerList { ... }

pub async fn create_solo_match_player(
    State(app_state): State<AppState>,
    Json(payload): Json<dto::MatchPlayer>,
) -> impl IntoResponse {

    let db = app_state.db.clone();
    println!("솔로 매치시작");
   println!("{:?}", payload);
   

}
pub async fn create_match_player(
    State(app_state): State<AppState>,
    Json(payload): Json<dto::MatchPlayer>,
) -> impl IntoResponse {
    let db = app_state.db.clone();
    println!("매치시작");
    println!("{:?}", payload);
    let dto::MatchPlayer { userid, status, .. } = payload;
    println!("[createMatchPlayer] request userid={}", userid);

    // 한국 시간(KST, UTC+9)을 서버에서 직접 생성해 DB에 저장합니다.
    let kst_offset = FixedOffset::east_opt(9 * 60 * 60).expect("valid offset");
    let match_at_kst = Utc::now().with_timezone(&kst_offset);
    let match_at_utc = match_at_kst.with_timezone(&Utc);

    let insert_sql = r#"
        INSERT INTO match_players 
        (userid, status , match_at)
        VALUES ($1, $2, $3)
    "#
    .to_owned();

    let stmt = Statement::from_sql_and_values(
        DatabaseBackend::Postgres,
        insert_sql,
        vec![userid.clone().into(), status.into(), match_at_utc.into()],
    );

    match db.execute(stmt).await {
        Ok(_) => {
            println!("match 성공 userid ={}", userid);
            println!("match 성공 status ={}", status);
            println!("match 성공 match_at(KST) ={}", match_at_kst);

            let partner_stmt = Statement::from_sql_and_values(
                DatabaseBackend::Postgres,
                r#"
                    SELECT userid
                    FROM match_players
                    WHERE status = 1
                      AND userid <> $1
                    ORDER BY match_at ASC
                    LIMIT 1
                "#,
                vec![userid.clone().into()],
            );

            match db.query_one(partner_stmt).await {
                Ok(Some(row)) => {
                    return handle_found_partner(&db, userid, row).await;
                }
                Ok(None) => {
                    let body = json!({ "success": 1, "userid": userid, "game_started": false });
                    (StatusCode::CREATED, Json(body))
                }
                Err(e) => {
                    eprintln!("매칭 파트너 조회 실패: {e}");
                    let body = json!({ "success": 0, "error": e.to_string() });
                    (StatusCode::INTERNAL_SERVER_ERROR, Json(body))
                }
            }
        }
        Err(e) => {
            eprintln!("[match_player] DB error: {e}");
            let body = json!({ "success": 0, "error": e.to_string() });
            (StatusCode::INTERNAL_SERVER_ERROR, Json(body))
        }
    }
}

fn respond_with_game_started(
    userid: String,
    game_summary: GameCreationResult,
) -> (StatusCode, Json<serde_json::Value>) {
    let game_value = game_summary.to_value();
    let body = json!({
        "success": 1,
        "userid": userid,
        "game_started": true,
        "game": game_value,
    });
    (StatusCode::OK, Json(body))
}

async fn handle_found_partner(
    db: &DatabaseConnection,
    userid: String,
    row: QueryResult,
) -> (StatusCode, Json<serde_json::Value>) {
    let partner_id = row.try_get::<String>("", "userid").unwrap_or_default();

    if partner_id.is_empty() {
        let body = json!({ "success": 1, "userid": userid, "game_started": false });
        return (StatusCode::CREATED, Json(body));
    }

    match start_holdem_game_for_players(db, &userid, &partner_id).await {
        Ok(game_summary) => respond_with_game_started(userid, game_summary),
        Err(e) => {
            eprintln!("자동 게임 생성 실패: {e}");
            let body = json!({
                "success": 0,
                "error": "Failed to start Hold'em game automatically",
            });
            (StatusCode::INTERNAL_SERVER_ERROR, Json(body))
        }
    }
}

pub async fn show_match_player_list(State(app_state): State<AppState>) -> impl IntoResponse {
    let db = app_state.db.clone();

    let select_sql = r#"
        SELECT * FROM match_players
        WHERE status = 1
        ORDER BY match_at DESC
    "#
    .to_owned();

    let stmt = Statement::from_sql_and_values(DatabaseBackend::Postgres, select_sql, vec![]);

    let result = db.query_all(stmt).await;
    println!("result = {:?}", result);
    match result {
        Ok(rows) => {
            let kst_offset = FixedOffset::east_opt(9 * 60 * 60).expect("valid offset");
            let players = rows
                .into_iter()
                .map(|row: QueryResult| {
                    let match_at_utc: DateTime<Utc> =
                        row.try_get("", "match_at").unwrap_or_else(|_| Utc::now());
                    let match_at_kst = match_at_utc.with_timezone(&kst_offset);

                    json!({
                        "userid": row.try_get::<String>("", "userid").unwrap_or_default(),
                        "status": row.try_get::<i32>("", "status").unwrap_or_default(),
                        "match_at": match_at_kst.to_rfc3339(),
                    })
                })
                .collect::<Vec<_>>();

            let body = json!({ "success": 1, "players": players });
            println!("body = {:?}", body);
            (StatusCode::OK, Json(body))
        }
        Err(e) => {
            eprintln!("[match_player] DB error: {e}");
            let body = json!({ "success": 0, "error": e.to_string() });
            (StatusCode::INTERNAL_SERVER_ERROR, Json(body))
        }
    }
}
