use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::{extract::State, Json};
use sea_orm::{ConnectionTrait, DatabaseBackend, QueryResult, Statement};
use serde_json::json;

use super::dto;
use super::holdem;
use crate::holdem::holdem::Game;
use crate::AppState;

// pub async fn start_game (
//     State(app_state): State<AppState>,
// ) -> () {
//     let db = app_state.db.clone();

//     let mut game = holdem::Game::default();

// }

// CREATE table game_player (
// 	gameid text primary key,
// 	player_id text,
// 	player_card1 VARCHAR(5),
// 	player_card2 VARCHAR(5),
// 	player_chips INT
// )

pub async fn start_game(
    State(app_state): State<AppState>,
    Json(payload): Json<dto::StartPlayers>,
) -> impl IntoResponse {
    let db = app_state.db.clone();
    let dto::StartPlayers {
        my_player_id,
        select_player_id,
    } = payload;
    /**
     * 	gameid text primary key,
    player_id text,
    player_card1 VARCHAR(5),
    player_card2 VARCHAR(5),

     */

    /*
    
CREATE table game_player (
	game_id text primary key,
	player_id text,
	player_card1 VARCHAR(5),
	player_card2 VARCHAR(5)
	
	
)


COMMIT;

create table holdem_games (
	game_id text primary key,
	community_cards1 VARCHAR(5),
	community_cards2 VARCHAR(5),
	community_cards3 VARCHAR(5),
	community_cards4 VARCHAR(5),
	community_cards5 VARCHAR(5),
	pot int,
	blind int,
	started_at  TIMESTAMPTZ NOT NULL DEFAULT NOW(), 
	end_at TIMESTAMPTZ
)
 */
    let insert_sql = r#"
        INSERT INTO game_player
        (gameid, player_id, player_card1, player_card2, player_chips)
        VALUES ($1, $2, $3, $4, $5)

    "#;
    let stmt = Statement::from_sql_and_values(
        DatabaseBackend::Postgres, 
        insert_sql, vec![]);

    let result = db.query_all(stmt).await;
    // fn main() {

    //     let mut player1 = Player {
    //         player_id: 1,
    //         player_name: "홍길동".to_owned(),
    //         ..Default::default()
    //     };
    //     let mut player2 = Player {
    //         player_id: 2,
    //         player_name: "홍진우".to_owned(),
    //         ..Default::default()
    //     };
    //     let mut game = Game {
    //         players_id: vec![player1.player_id, player2.player_id],
    //         pot: 0,
    //         deck: Deck::default(),
    //         community_cards: Vec::new(),
    //     };
    //     println!("게임 시작!");
    //     println!("플레이어 1 카드 : {:?}", player1.cards);
    //     println!("플레이어 2 카드 : {:?}", player2.cards);

    //     player1.cards = game.start_dealing();
    //     player2.cards = game.start_dealing();
    //     player1.game_played = 1;
    //     player2.game_played = 1;
    //     game.bet(3000, &mut player1);
    //     game.bet(200, &mut player2);
    //     game.open_flop();
    //     game.open_turn();
    //     game.open_river();
    //     let mut players: Vec<&Player> = vec![&player1, &player2];

    //     let ranks = &game.get_winner(&players);
    //     let mut players_mut: Vec<&mut Player> = vec![&mut player1, &mut player2];

    //     game.give_pot_for_winner(&ranks, &mut players_mut);
    //     println!("플레이어 1 남은 돈 : {}", player1.money);
    //     println!("플레이어 2 남은 돈 : {}", player2.money);
    // }
}

// pub async fn get_my_hand(
//     State(app_state): State<AppState>,
//     Json(payload): Json<dto::PlayerCards>,
// ) -> impl IntoResponse {
//     let db = app_state.db.clone();

//     let dto::PlayerCards {
//         player_id,
//         player_cards
//     } = payload;

//     let holdem::Game {
//         players_id,
//         pot,
//         community_cards,
//         deck,

//     } = game;
// }
