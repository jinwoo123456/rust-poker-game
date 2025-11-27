pub use rs_poker::core::{Card, Deck, Rank, Rankable};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerCards {
    pub player_id: String,
    pub player_cards: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StartPlayers {
    pub player_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommunityCards {
    pub community_cards: Vec<String>,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StratGame {
    pub player1_id: String,
    pub player1_money: i32,
    pub player1_card1: String,
    pub player1_card2: String,
    pub player2_id: String,
    pub player2_money: i32,
    pub player2_card1: String,
    pub player2_card2: String,
    pub blind: i32,
    pub pot: i32,
    pub community_card1 : String,
    pub community_card2 : String,
    pub community_card3 : String,
    pub community_card4 : String,
    pub community_card5 : String,
}