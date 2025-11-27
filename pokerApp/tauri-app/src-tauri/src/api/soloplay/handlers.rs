use super::dto;
use super::holdem_algo;
use super::agent;
use crate::AppState;
use crate::api::soloplay::holdem_algo::GameTrait;
use crate::api::soloplay::agent::Bot;
pub use rs_poker::core::{Card, Deck, Rank, Rankable};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::{
    extract::{Path, State},
    Json,
};
pub use rand::rng;
use sea_orm::{ConnectionTrait, DatabaseBackend, DatabaseConnection, DbErr, QueryTrait, Statement};
use serde_json::json;
use async_trait::async_trait;



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
         let mut bot =  Bot {
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
         let  mut game =  holdem_algo::Game {
            players_id: vec![player.player_id.clone(), bot.bot_id.clone()],
            community_cards: Vec::new(),
            deck: Deck::default(),
            pot: 0,
        };
        player.cards = game.start_dealing();
        bot.cards = game.start_dealing();
        
        // 공유패 미리 받아놓기
        game.open_flop();
        game.open_turn();
        game.open_river();
        game.pot = 0;
        println!("플랍카드 : {} {} {} 턴카드 :  {} 리버카드:  {} ",
         game.community_cards[0],
          game.community_cards[1], 
          game.community_cards[2],
          game.community_cards[3], 
          game.community_cards[4]);


          // 쿼리 =================
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
                            blind = EXCLUDED.blind,
                            pot = 0
    "#;

    let startgame = dto::StratGame {
        game_id: "game_1".to_owned(),
        community_card1: format!("{}", game.community_cards[0]),
        community_card2: format!("{}", game.community_cards[1]),
        community_card3: format!("{}", game.community_cards[2]),
        community_card4: format!("{}", game.community_cards[3]),
        community_card5: format!("{}", game.community_cards[4]),
        blind: 200,
        pot : 0,
    };
    let game_stmt = Statement::from_sql_and_values(
        DatabaseBackend::Postgres,
        insert_holdem_game_sql,
        vec![
            startgame.game_id.clone().into(),
            startgame.community_card1.clone().into(),
            startgame.community_card2.clone().into(),
            startgame.community_card3.clone().into(),
            startgame.community_card4.clone().into(),
            startgame.community_card5.clone().into(),
            startgame.blind.into(),
        ],
    );
    let game_result = db.execute(game_stmt).await.expect("게임 시작 쿼리 오류");
    println!(
        "holdem_games inserted rows: {}",
        game_result.rows_affected()
    );
 //
         let body = json!({
            "success": 1,
            "player_id": player.player_id,
            "player_cards": format!("{:?}", player.cards),
            "bot_id": bot.bot_id,
            "bot_name": bot.bot_name,
         });
         
        (StatusCode::OK, Json(body))
    }
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