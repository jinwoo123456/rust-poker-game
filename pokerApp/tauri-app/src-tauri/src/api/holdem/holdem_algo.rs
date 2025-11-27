pub use rand::rng;
pub use rs_poker::core::{Card, Deck, Rank, Rankable};
use std::cmp::Ordering;

#[derive(Debug, Clone)]
pub struct Game {
    pub players_id: Vec<i32>,
    pub pot: i32,
    pub deck: Deck,
    pub community_cards: Vec<Card>,
}

#[derive(Debug, Clone)]
pub struct Player {
    pub player_id: i32,
    pub player_name: String,
    pub cards: Vec<Card>,
    pub money: i32,
    pub game_played: i32,
}
impl Default for Game {
    fn default() -> Self {
        Self {
            players_id: Vec::new(),
            pot: 0,
            deck: Deck::new(),
            community_cards: Vec::new(),
        }
    }
}

// Player 기본값
impl Default for Player {
    fn default() -> Self {
        Self {
            player_id: 0,
            player_name: String::new(),
            cards: Vec::new(),
            money: 20_000,
            game_played: 0,
        }
    }
}

fn eq_hands(player1_rank: &Rank, player2_rank: &Rank) -> Ordering {
    player1_rank.cmp(player2_rank)
}

// 인터페이스
pub trait GameTrait {
    // 시작시 플레이어들에게 카드 나눠주기
    fn start_dealing(&mut self) -> Vec<Card>;
    // 첫 3장 오픈
    fn open_flop(&mut self);
    // 4번째 카드 오픈
    fn open_turn(&mut self);
    // 5번째 카드 오픈 (마지막)
    fn open_river(&mut self);
    // 핸드 비교 (누가 이겼는지)
    fn get_winner(&mut self, players: &Vec<&Player>) -> Vec<(i32, Rank)>;

    // 승자한테 돈 몰아주기
    fn give_pot_for_winner(&mut self, winners: &Vec<(i32, Rank)>, players: &mut Vec<&mut Player>);
    // 돈 배팅
    fn bet(&mut self, bet_money: i32, player: &mut Player);
    // 폴드 ( 게임 포기 )
    fn fold(&mut self, player: &mut Player);
}

pub trait PlayerTrait {
    // 내 핸드 평가 ( 내 카드 순위 )
    fn my_cards_rank(&self, community_cards: &Vec<Card>) -> Rank;
}

impl PlayerTrait for Player {
    fn my_cards_rank(&self, community_cards: &Vec<Card>) -> Rank {
        let mut my_cards = self.cards.clone();
        println!("내 카드 조합  : {:?}", my_cards);
        my_cards.extend(community_cards.clone());

        let cards_rank = my_cards.rank();
        println!("내 카드 순위  : {:?}", cards_rank);
        cards_rank
    }
}

// 인터페이스 구현
impl GameTrait for Game {
    fn start_dealing(&mut self) -> Vec<Card> {
        let mut rng = rng();
        let cards = vec![
            self.deck.deal(&mut rng).unwrap(),
            self.deck.deal(&mut rng).unwrap(),
        ];
        println!("플레이어 카드받기후 : {:?}", cards);
        cards
    }
    // 첫 3장 오픈 ( 홀덤 룰 )
    fn open_flop(&mut self) {
        let mut rng = rng();
        for _ in 0..3 {
            self.community_cards.push(self.deck.deal(&mut rng).unwrap());
            println!("플랍 : {:?}", &self.community_cards);
        }
    }
    // 4번째 카드 오픈
    fn open_turn(&mut self) {
        let mut rng = rng();
        self.community_cards.push(self.deck.deal(&mut rng).unwrap());
        println!("턴 : {:?}", &self.community_cards);
    }
    //5번째 카드 오픈
    fn open_river(&mut self) {
        let mut rng = rng();
        self.community_cards.push(self.deck.deal(&mut rng).unwrap());
        println!("리버 : {:?}", &self.community_cards);
    }

    // 카드 비교 (누가 이겼는지)
    fn get_winner(&mut self, players: &Vec<&Player>) -> Vec<(i32, Rank)> {
        let ranks: Vec<(i32, Rank)> = players
            .iter()
            .map(|player| {
                let rank = player.my_cards_rank(&self.community_cards);
                (player.player_id, rank)
            })
            .collect();
        let best_rank = ranks.iter().map(|(_, rank)| *rank).max().unwrap();

        let winners: Vec<(i32, Rank)> = ranks
            .into_iter()
            .filter(|(_, rank)| *rank == best_rank)
            .collect();

        println!("승자: {:?}", winners);
        winners
    }
    // 머니 배팅
    fn bet(&mut self, bet_money: i32, player: &mut Player) {
        if player.game_played == 0 {
            println!("폴드하셨거나 팟에 참여를 안하셨습니다.");
            return;
        }
        println!("{}가 {}만큼 배팅 했습니다.", player.player_name, bet_money);
        println!("배팅 전 남은 돈: {}", player.money);
        player.money -= bet_money;
        println!("배팅 후 남은 돈: {}", player.money);
        println!("현재 팟: {}", self.pot + bet_money);
        self.pot += bet_money;
    }
    // 폴드 ( 게임 포기 )
    fn fold(&mut self, player: &mut Player) {
        player.game_played = 0;
        println!("{}가 도망쳤습니다.", player.player_name);
    }
    fn give_pot_for_winner(&mut self, winners: &Vec<(i32, Rank)>, players: &mut Vec<&mut Player>) {
        let pot_share = self.pot / winners.len() as i32;
        for (winner_id, _) in winners {
            if let Some(winner) = players.iter_mut().find(|p| p.player_id == *winner_id) {
                winner.money += pot_share;
                println!(
                    "{}님이 {}만큼의 돈을 받았습니다.",
                    winner.player_name, pot_share
                );
            }
        }
        self.pot = 0;
    }
}
