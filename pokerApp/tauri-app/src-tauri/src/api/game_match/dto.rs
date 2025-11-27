use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MatchPlayer {
    pub userid: String,
    pub status: i32,
    pub match_at: chrono::DateTime<Utc>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MatchPlayerList {
    pub players: Vec<MatchPlayer>,
}

// impl Default for MatchRequest {
//     fn default() -> Self {
//         MatchRequest {
//             userid: String::new(),
//             match_state: false,
//         }
//     }
// }
