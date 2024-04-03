mod card;
mod individual;
mod shoe;
use shoe::Shoe;

use individual::{Dealer, Player};

use self::individual::Individual;


#[derive(Debug)]
pub struct Game {
    shoe: Shoe,
    dealer: Dealer,
    player: Vec<Player>,
}

impl Game {
    pub fn new(number_of_decks: u8, players: Vec<String>, shuffle: bool) -> Self {
        Self {
            shoe: Shoe::new(number_of_decks, shuffle),
            dealer: Dealer::new(),
            player: players.iter().map(|name| Player::new(name.clone())).collect(),
        }
    }
}
