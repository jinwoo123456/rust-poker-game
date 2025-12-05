use super::agent;
use super::dto;
use super::holdem_algo;
use crate::api::soloplay::agent::Bot;
use crate::api::soloplay::holdem_algo::GameTrait;
use crate::AppState;
use async_trait::async_trait;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::{
    extract::{Path, State},
    Json,
};
use std::str::FromStr;

pub use rand::rng;
pub use rs_poker::core::{Card, Deck, Rank, Rankable};
use sea_orm::{
    ConnectionTrait, DatabaseBackend, DatabaseConnection, DbErr, QueryTrait, Statement, TryGetable,
};
use serde_json::json;

#[derive(Debug, Clone)]
pub struct CreateGame {
    pub game_id: String,
    pub community_cards1: String,
    pub community_cards2: String,
    pub community_cards3: String,
    pub community_cards4: String,
    pub community_cards5: String,
    pub blind: i32,
    pub pot: i32,
}

#[derive(Debug, Clone)]
pub struct Player {
    pub player_id: String,
    pub player_name: String,
    pub cards: Vec<Card>,
    pub money: i32,
    pub game_played: i32,
}

#[derive(Debug, Clone)]
struct GameSetup {
    game: CreateGame,
    player1: Player,
    player_ai: Bot,
}

#[derive(Debug, Clone)]
pub struct GameCreationResult {
    pub game_id: String,
    pub player1_id: String,
    pub player2_id: String,
    pub blind: i32,
}

impl GameCreationResult {
    pub fn to_value(&self) -> serde_json::Value {
        json!({
            "success": 1,
            "game_id": self.game_id,
            "player1_id": self.player1_id,
            "player2_id": self.player2_id,
            "blind": self.blind,
        })
    }
}
#[async_trait]
pub trait GameHandlersTrait {
    async fn start_solo_game(
        State(app_state): State<AppState>,
        Json(payload): Json<dto::StartPlayers>,
    ) -> impl IntoResponse;

    // async fn player_action(
    //     State(app_state): State<AppState>,
    //     Json(payload): Json<dto::PlayerAction>,
    // ) -> impl IntoResponse;

    // async fn get_cards(
    //     State(app_state): State<AppState>,
    //     Path(player_id): Path<String>,
    // ) -> impl IntoResponse;
    // async fn open_flop(
    //     State(app_state): State<AppState>,
    //     Path(game_id): Path<String>,
    // ) -> impl IntoResponse;
    // async fn open_turn(
    //     State(app_state): State<AppState>,
    //     Path(game_id): Path<String>,
    // ) -> impl IntoResponse;
    // async fn open_river(
    //     State(app_state): State<AppState>,
    //     Path(game_id): Path<String>,
    // ) -> impl IntoResponse;
}
pub struct GetWinnerHandler;
#[async_trait]
pub trait GetWinnerTrait {
    async fn get_winner(
        State(app_state): State<AppState>,
        Json(payload): Json<dto::getWinnerReq>,
    ) -> impl IntoResponse;
}

#[async_trait]
impl GetWinnerTrait for GetWinnerHandler {
    async fn get_winner(
        State(app_state): State<AppState>,
        Json(payload): Json<dto::getWinnerReq>,
    ) -> impl IntoResponse {
        let player_cards: Vec<Card> = payload
            .playerCards
            .iter()
            .map(|s| Card::try_from(s.as_str()).expect("플레이어 카드 형변환 실패"))
            .collect();

        let bot_cards: Vec<Card> = payload
            .botCards
            .iter()
            .map(|s| Card::try_from(s.as_str()).expect("봇 카드 형변환 실패"))
            .collect();

        let community_cards: Vec<Card> = payload
            .communityCards
            .iter()
            .map(|s| Card::try_from(s.as_str()).expect("커뮤니티 카드 형변환 실패"))
            .collect();

        println!("플레이어 카드 : {:?}", player_cards);
        println!("봇 카드 : {:?}", bot_cards);
        println!("커뮤니티 카드 : {:?}", community_cards);
        // 2) 승자 계산
        let raw_winners = holdem_algo::check_winner(&player_cards, &bot_cards, &community_cards);
        println!("승자 결과 (raw) : {:?}", raw_winners);

        let winner_details: Vec<dto::WinnerDetail> = raw_winners
            .iter()
            .map(|(player_id, rank)| dto::WinnerDetail {
                player_id: *player_id,
                rank: format!("{:?}", rank),
            })
            .collect();

        let result = if winner_details.len() == 1 {
            match winner_details[0].player_id {
                1 => "player".to_string(),
                2 => "bot".to_string(),
                _ => "unknown".to_string(),
            }
        } else {
            "draw".to_string()
        };

        let resp_body = dto::WinnerResponse {
            result,
            winners: winner_details,
        };

        (StatusCode::OK, Json(resp_body))
    }
}

pub struct GameHandlers;
#[async_trait]
impl GameHandlersTrait for GameHandlers {
    async fn start_solo_game(
        State(app_state): State<AppState>,
        Json(payload): Json<dto::StartPlayers>,
    ) -> impl IntoResponse {
        let db = app_state.db.clone();

        //=============가진 금액 설정 ===============

        //=============패 주기 =============
        // 게임 세팅 ( 유저 , 봇  지정)
        let mut bot = Bot {
            bot_id: "1".to_owned(),
            bot_name: String::from("jinwoo_bot"),
            cards: Vec::new(),
            money: 20000,
            game_played: 1,
        };
        let mut player = Player {
            player_id: payload.player_id,
            player_name: "플레이어".to_owned(),
            cards: Vec::new(),
            money: 20000,
            game_played: 1,
        };
        // 게임 생성 ( 한판마다 다시 생성됨 . )
        let mut game = holdem_algo::Game {
            players_id: vec![player.player_id.clone(), bot.bot_id.clone()],
            community_cards: Vec::new(),
            deck: Deck::default(),
            pot: 0,
        };
        player.cards = game.start_dealing();
        bot.cards = game.start_dealing();
        println!("봇카드 {:?}", bot.cards);

        // 공유패 미리 받아놓기
        game.open_flop();
        game.open_turn();
        game.open_river();
        game.pot = 0;
        println!(
            "플랍카드 : {} {} {} 턴카드 :  {} 리버카드:  {} ",
            game.community_cards[0],
            game.community_cards[1],
            game.community_cards[2],
            game.community_cards[3],
            game.community_cards[4]
        );

        // 쿼리 =================
        let insert_holdem_game_sql = r#"
            INSERT INTO holdem_games (
                player1_id, player1_money, player1_card1, player1_card2,
                player2_id, player2_money, player2_card1, player2_card2,
                community_cards1, community_cards2, community_cards3, community_cards4, community_cards5,
                blind, pot
            )
            VALUES (
                $1, $2, $3, $4,
                $5, $6, $7, $8,
                $9, $10, $11, $12, $13,
                $14, 0
            )
            RETURNING game_id
            "#;
        let startgame = dto::StratGame {
            player1_id: player.player_id.clone(),
            player1_money: player.money,
            player1_card1: format!("{}", player.cards[0]),
            player1_card2: format!("{}", player.cards[1]),
            player2_id: bot.bot_id.clone(),
            player2_money: bot.money,
            player2_card1: format!("{}", bot.cards[0]),
            player2_card2: format!("{}", bot.cards[1]),
            community_card1: format!("{}", game.community_cards[0]),
            community_card2: format!("{}", game.community_cards[1]),
            community_card3: format!("{}", game.community_cards[2]),
            community_card4: format!("{}", game.community_cards[3]),
            community_card5: format!("{}", game.community_cards[4]),
            blind: 200,
            pot: 0,
        };
        let game_stmt: Statement = Statement::from_sql_and_values(
            DatabaseBackend::Postgres,
            insert_holdem_game_sql,
            vec![
                // startgame.game_id.clone().into(),
                startgame.player1_id.clone().into(),
                startgame.player1_money.into(),
                startgame.player1_card1.clone().into(),
                startgame.player1_card2.clone().into(),
                startgame.player2_id.clone().into(),
                startgame.player2_money.into(),
                startgame.player2_card1.clone().into(),
                startgame.player2_card2.clone().into(),
                startgame.community_card1.clone().into(),
                startgame.community_card2.clone().into(),
                startgame.community_card3.clone().into(),
                startgame.community_card4.clone().into(),
                startgame.community_card5.clone().into(),
                startgame.blind.into(),
            ],
        );
        let row_opt = db.query_one(game_stmt).await.expect("게임 시작 쿼리 오류");

        let game_id: i64 = if let Some(row) = row_opt {
            // 첫 번째 인자 "" = 테이블 alias (없으면 빈 문자열)
            row.try_get("", "game_id").expect("game_id 가져오기 실패")
        } else {
            eprintln!("게임 생성 실패: game_id row 없음");
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "success": 0,
                    "message": "게임 생성 실패",
                })),
            );
        };

        println!("생성된 game_id: {}", game_id);
        let body = json!({
             "game_id": game_id,
           "success": 1,
           "player_id": player.player_id,
           "player_cards": format!("{:?}", player.cards),
           "bot_cards": format!("{:?}", bot.cards),
           "comunity_cards": format!("{:?}", game.community_cards),
           "bot_id": bot.bot_id,
           "bot_name": bot.bot_name,
            "blind": startgame.blind,
            "player_money": player.money,
            "bot_money": bot.money,
            "pot": game.pot,
        });

        (StatusCode::OK, Json(body))
    }

    //액션은 프론트에서 처리
    // async fn player_action(
    //     State(app_state): State<AppState>,
    //     Json(payload): Json<dto::PlayerAction>,
    // ) -> impl IntoResponse {
    //     // let db = app_state.db.clone();

    //     // let select_get_player_money_sql = r#"
    //     //     SELECT player1_money, player2_money, pot FROM holdem_games
    //     //     WHERE player1_id = $1 OR player2_id = $1
    //     //     "#;
    // }
}

/*
async fn persist_game_setup(db: &DatabaseConnection, setup: &GameSetup) -> Result<(), DbErr> {
    let insert_holdem_game_sql = r#"
        INSERT INTO holdem_games
        (game_id, community_cards1, community_cards2, community_cards3, community_cards4, community_cards5 , blind)
        VALUES ($1, $2, $3, $4, $5, $6 , $7)
                    ON CONFLICT (game_id) DO UPDATE SET
                            community_cards1 = EXCLUDED.community_cards1,
                            community_cards2 = EXCLUDED.community_cards2,
                            community_cards3 = EXCLUDED.community_cards3,
                            community_cards4 = EXCLUDED.community_cards4,
                            community_cards5 = EXCLUDED.community_cards5,
                            blind = EXCLUDED.blind
    "#;

    let game_stmt = Statement::from_sql_and_values(
        DatabaseBackend::Postgres,
        insert_holdem_game_sql,
        vec![
            setup.game.game_id.clone().into(),
            setup.game.community_cards1.clone().into(),
            setup.game.community_cards2.clone().into(),
            setup.game.community_cards3.clone().into(),
            setup.game.community_cards4.clone().into(),
            setup.game.community_cards5.clone().into(),
            setup.game.blind.into(),
        ],
    );
    let game_result = db.execute(game_stmt).await?;
    println!(
        "holdem_games inserted rows: {}",
        game_result.rows_affected()
    );

    let insert_player_sql = r#"
        INSERT INTO game_player
        (game_id, player_id, player_card1, player_card2, player_chips)
        VALUES ($1, $2, $3, $4, $5)
        ON CONFLICT (game_id, player_id) DO UPDATE SET
            player_card1 = EXCLUDED.player_card1,
            player_card2 = EXCLUDED.player_card2,
            player_chips = EXCLUDED.player_chips
    "#;

    let player1_stmt = Statement::from_sql_and_values(
        DatabaseBackend::Postgres,
        insert_player_sql,
        vec![
            setup.player1.game_id.clone().into(),
            setup.player1.player_id.clone().into(),
            setup.player1.player_card1.clone().into(),
            setup.player1.player_card2.clone().into(),
            setup.player1.player_chips.into(),
        ],
    );
    let player1_result = db.execute(player1_stmt).await?;
    println!(
        "game_player inserted rows (player1): {}",
        player1_result.rows_affected()
    );

    let player2_stmt = Statement::from_sql_and_values(
        DatabaseBackend::Postgres,
        insert_player_sql,
        vec![
            setup.player2.game_id.clone().into(),
            setup.player2.player_id.clone().into(),
            setup.player2.player_card1.clone().into(),
            setup.player2.player_card2.clone().into(),
            setup.player2.player_chips.into(),
        ],
    );
    let player2_result = db.execute(player2_stmt).await?;
    println!(
        "game_player inserted rows (player2): {}",
        player2_result.rows_affected()
    );

    let cleanup_stmt = Statement::from_sql_and_values(
        DatabaseBackend::Postgres,
        r#"DELETE FROM match_players WHERE userid = $1 OR userid = $2"#,
        vec![
            setup.player1.player_id.clone().into(),
            setup.player2.player_id.clone().into(),
        ],
    );
    let cleanup_result = db.execute(cleanup_stmt).await?;
    println!(
        "match_players deleted rows: {}",
        cleanup_result.rows_affected()
    );

    Ok(())
}

    */
