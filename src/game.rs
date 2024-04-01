mod card;
mod individual;
mod shoe;
use shoe::Shoe;

use individual::{Dealer, Player};

pub struct Game {
    shoe: Shoe,
    player: Vec<Player>,
    dealer: Dealer,
}
