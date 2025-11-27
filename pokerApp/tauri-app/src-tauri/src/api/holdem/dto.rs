use rs_poker::core::Card;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerCards {
    pub player_id: String,
    pub player_cards: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StartPlayers {
    pub player_id: String,
    pub select_player_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommunityCards {
    pub community_cards: Vec<String>,
}
