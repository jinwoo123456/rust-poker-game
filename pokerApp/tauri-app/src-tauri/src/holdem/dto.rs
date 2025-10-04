use rs_poker::core::Card;
use serde::{Deserialize, Serialize};



#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerCards {
    pub player_id: i32,
    pub player_cards: Vec<String>,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StartPlayers {
    pub my_player_id:i32,
    pub select_player_id:i32,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommunityCards {
    pub community_cards: Vec<String>,
}