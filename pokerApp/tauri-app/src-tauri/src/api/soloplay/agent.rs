use super::handlers::CreateGame;
pub use rand::rng;
pub use rand::{rngs::StdRng, Rng, SeedableRng};
pub use rs_poker::core::{Card, Deck, Rank, Rankable};
use std::cmp::Ordering;
#[derive(Debug, Clone)]
pub struct Bot {
    pub bot_id: String,
    pub bot_name: String,
    pub cards: Vec<Card>,
    pub money: i32,
    pub game_played: i32,
}
// 봇 기본값
impl Default for Bot {
    fn default() -> Self {
        Self {
            bot_id: String::new(),
            bot_name: String::new(),
            cards: Vec::new(),
            money: 20000,
            game_played: 0,
        }
    }
}
pub trait BotTrait {
    // 0 : 체크 / 콜, 1 : 베팅 / 레이즈, 2 : 폴드

    fn bot_action(&mut self, enermy_action: i32, enemy_bet: i32) -> Vec<i32>;
    fn bet_money(&mut self) -> i32;
}

impl BotTrait for Bot {
    fn bot_action(&mut self, enermy_action: i32, enemy_bet: i32) -> Vec<i32> {
        let mut rng = rng();
        let action = rng.random_range(0..3);
        let mut bet = 0;
        if enemy_bet == 1 {
            if action == 1 {
                bet = rng.random_range(200..self.money);
                println!("Bot {} bets {}", self.bot_id, bet);
            }
        }
        // 폴드
        if action == 2 {
            println!("폴드 선택  : {} ", action);
        }
        // 상대가 배팅이 없을 때
        else if enemy_bet == 0 {
            // 0나오면 체크로 넘기기
            if action == 0 {
                return vec![action, 0];
            }
            // 배팅 금액 랜덤
            else if action == 1 {
                bet = rng.random_range(200..self.money / 2); // 일단 가진돈의 절반만 배팅
                println!("Bot {} bets {}", self.bot_id, bet);
                return vec![action, bet];
            }
            //상대 배팅이 없으므로 폴드 x 체크로 넘어감
            else if action == 2 {
                return vec![0, 0];
            }
        } else if enermy_action == 1 {
            if action == 0 {
                println!("Bot {} calls {}", self.bot_id, enemy_bet);
                self.money -= enemy_bet;
                return vec![action, enemy_bet];
            }
        }
        vec![action, bet]
    }
    fn bet_money(&mut self) -> i32 {
        3
    }
}
