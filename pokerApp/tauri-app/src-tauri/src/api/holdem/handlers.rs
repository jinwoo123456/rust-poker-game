use super::dto;
use super::holdem_algo as holdem;
use crate::AppState;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::{
    extract::{Path, State},
    Json,
};
pub use rand::rng;
pub use rs_poker::core::{Deck, Rank, Rankable};
use sea_orm::{ConnectionTrait, DatabaseBackend, DatabaseConnection, DbErr, QueryTrait, Statement};
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
}
#[derive(Debug, Clone)]
pub struct CreatePlayer {
    pub game_id: String,
    pub player_id: String,
    pub player_card1: String,
    pub player_card2: String,
    pub player_chips: i32,
}

#[derive(Debug, Clone)]
struct GameSetup {
    game: CreateGame,
    player1: CreatePlayer,
    player2: CreatePlayer,
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





async fn ensure_game_player_schema(db: &DatabaseConnection) -> Result<(), DbErr> {
    let ddl = r#"
        DO $$
        DECLARE
            column_exists BOOLEAN;
        BEGIN
            EXECUTE '
                CREATE TABLE IF NOT EXISTS game_player (
                    game_id TEXT NOT NULL,
                    player_id TEXT NOT NULL,
                    player_card1 VARCHAR(5),
                    player_card2 VARCHAR(5),
                    player_chips INT DEFAULT 0
                )
            ';

            EXECUTE 'ALTER TABLE game_player DROP CONSTRAINT IF EXISTS game_player_pkey';
            EXECUTE 'ALTER TABLE game_player DROP CONSTRAINT IF EXISTS game_player_pk';
            EXECUTE 'ALTER TABLE game_player DROP CONSTRAINT IF EXISTS game_player_composite_pkey';

            SELECT EXISTS (
                SELECT 1
                FROM information_schema.columns
                WHERE table_name = 'game_player'
                  AND column_name = 'player_chips'
            ) INTO column_exists;

            IF NOT column_exists THEN
                EXECUTE 'ALTER TABLE game_player ADD COLUMN player_chips INT DEFAULT 0';
            END IF;

            SELECT EXISTS (
                SELECT 1
                FROM information_schema.columns
                WHERE table_name = 'game_player'
                  AND column_name = 'player_card1'
            ) INTO column_exists;

            IF NOT column_exists THEN
                EXECUTE 'ALTER TABLE game_player ADD COLUMN player_card1 VARCHAR(5)';
            END IF;

            SELECT EXISTS (
                SELECT 1
                FROM information_schema.columns
                WHERE table_name = 'game_player'
                  AND column_name = 'player_card2'
            ) INTO column_exists;

            IF NOT column_exists THEN
                EXECUTE 'ALTER TABLE game_player ADD COLUMN player_card2 VARCHAR(5)';
            END IF;

            BEGIN
                EXECUTE 'ALTER TABLE game_player ADD CONSTRAINT game_player_composite_pkey PRIMARY KEY (game_id, player_id)';
            EXCEPTION
                WHEN duplicate_table THEN
                    RAISE NOTICE ''game_player already has composite primary key'';
                WHEN duplicate_object THEN
                    RAISE NOTICE ''game_player primary key already exists'';
            END;
        END$$;
    "#;

    let stmt = Statement::from_string(DatabaseBackend::Postgres, ddl.to_owned());
    db.execute(stmt).await.map(|_| ())
}

fn generate_game_setup(player_id: &str, select_player_id: &str) -> GameSetup {
    let mut deck = holdem::Deck::default();
    let mut rng = rng();

    let community_cards = [
        deck.deal(&mut rng).unwrap().to_string(),
        deck.deal(&mut rng).unwrap().to_string(),
        deck.deal(&mut rng).unwrap().to_string(),
        deck.deal(&mut rng).unwrap().to_string(),
        deck.deal(&mut rng).unwrap().to_string(),
    ];
    println!("Community Cards: {:?}", community_cards);

    let player1_cards = [
        deck.deal(&mut rng).unwrap().to_string(),
        deck.deal(&mut rng).unwrap().to_string(),
    ];
    println!("Player 1 Cards: {:?}", player1_cards);

    let player2_cards = [
        deck.deal(&mut rng).unwrap().to_string(),
        deck.deal(&mut rng).unwrap().to_string(),
    ];
    println!("Player 2 Cards: {:?}", player2_cards);

    let game_id = format!("{}{}", player_id, select_player_id);

    let game = CreateGame {
        game_id: game_id.clone(),
        community_cards1: community_cards[0].clone(),
        community_cards2: community_cards[1].clone(),
        community_cards3: community_cards[2].clone(),
        community_cards4: community_cards[3].clone(),
        community_cards5: community_cards[4].clone(),
        blind: 200,
    };

    let player1 = CreatePlayer {
        game_id: game_id.clone(),
        player_id: player_id.to_string(),
        player_card1: player1_cards[0].clone(),
        player_card2: player1_cards[1].clone(),
        player_chips: 20000,
    };

    let player2 = CreatePlayer {
        game_id,
        player_id: select_player_id.to_string(),
        player_card1: player2_cards[0].clone(),
        player_card2: player2_cards[1].clone(),
        player_chips: 20000,
    };

    GameSetup {
        game,
        player1,
        player2,
    }
}

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

pub async fn start_holdem_game_for_players(
    db: &DatabaseConnection,
    player_id: &str,
    select_player_id: &str,
) -> Result<GameCreationResult, DbErr> {
    if let Err(e) = ensure_game_player_schema(db).await {
        eprintln!("game_player 스키마 점검 실패: {e}");
        return Err(e);
    }

    let setup = generate_game_setup(player_id, select_player_id);

    persist_game_setup(db, &setup).await?;

    Ok(GameCreationResult {
        game_id: setup.game.game_id.clone(),
        player1_id: setup.player1.player_id.clone(),
        player2_id: setup.player2.player_id.clone(),
        blind: setup.game.blind,
    })
}

pub async fn start_game(
    State(app_state): State<AppState>,
    Json(payload): Json<dto::StartPlayers>,
) -> impl IntoResponse {
    let db = app_state.db.clone();

    match start_holdem_game_for_players(&db, &payload.player_id, &payload.select_player_id).await
    {
        Ok(summary) => {
            println!("게임 및 플레이어 생성 성공");
            let body = summary.to_value();
            (StatusCode::CREATED, Json(body))
        }
        Err(err) => {
            eprintln!("게임 및 플레이어 생성 실패: {err}");
            let body = json!({
                "success": 0,
                "error": "DB error during game or player creation",
            });
            (StatusCode::INTERNAL_SERVER_ERROR, Json(body))
        }
    }
}

pub async fn get_active_game(
    State(app_state): State<AppState>,
    Path(player_id): Path<String>,
) -> impl IntoResponse {
    let db = app_state.db.clone();

    if let Err(e) = ensure_game_player_schema(&db).await {
        eprintln!("game_player 스키마 점검 실패: {e}");
    }

    let trimmed_id = player_id.trim().to_owned();

    if trimmed_id.is_empty() {
        let body = json!({ "success": 0, "error": "player_id is required" });
        return (StatusCode::BAD_REQUEST, Json(body));
    }

    let select_sql = r#"
        SELECT 
            gp.game_id,
            gp.player_card1,
            gp.player_card2,
            gp.player_chips,
            hg.community_cards1,
            hg.community_cards2,
            hg.community_cards3,
            hg.community_cards4,
            hg.community_cards5,
            hg.blind
        FROM game_player gp
        JOIN holdem_games hg ON gp.game_id = hg.game_id
        WHERE gp.player_id = $1
        ORDER BY gp.game_id DESC
        LIMIT 1
    "#;

    let stmt = Statement::from_sql_and_values(
        DatabaseBackend::Postgres,
        select_sql,
        vec![trimmed_id.clone().into()],
    );

    match db.query_one(stmt).await {
        Ok(Some(row)) => {
            let body = json!({
                "success": 1,
                "game": {
                    "game_id": row.try_get::<String>("", "game_id").unwrap_or_default(),
                    "player_id": trimmed_id,
                    "player": {
                        "card1": row.try_get::<String>("", "player_card1").unwrap_or_default(),
                        "card2": row.try_get::<String>("", "player_card2").unwrap_or_default(),
                        "chips": row.try_get::<i32>("", "player_chips").unwrap_or_default(),
                    },
                    "community_cards": [
                        row.try_get::<String>("", "community_cards1").unwrap_or_default(),
                        row.try_get::<String>("", "community_cards2").unwrap_or_default(),
                        row.try_get::<String>("", "community_cards3").unwrap_or_default(),
                        row.try_get::<String>("", "community_cards4").unwrap_or_default(),
                        row.try_get::<String>("", "community_cards5").unwrap_or_default(),
                    ],
                    "blind": row.try_get::<i32>("", "blind").unwrap_or_default(),
                }
            });
            (StatusCode::OK, Json(body))
        }
        Ok(None) => {
            let body = json!({ "success": 0, "game": serde_json::Value::Null });
            (StatusCode::OK, Json(body))
        }
        Err(e) => {
            eprintln!("활성 게임 조회 실패: {e}");
            let body = json!({ "success": 0, "error": e.to_string() });
            (StatusCode::INTERNAL_SERVER_ERROR, Json(body))
        }
    }
}
// let dto::StartPlayers {
//     player_id,
//     select_player_id,
// } = payload;

// let insert_sql = r#"
//     INSERT INTO game_player
//     (game_id, player_id, player_card1, player_card2, player_chips)
//     VALUES ($1, $2, $3, $4, $5)

// "#;

//     * 	game_id text primary key,
//    player_id text,
//    player_card1 VARCHAR(5),
//    player_card2 VARCHAR(5),

/*]

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
