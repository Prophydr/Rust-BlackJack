use super::card::Card;

#[derive(Debug)]
pub struct Player {
    pub cards: Vec<Card>,
    pub name: String
}

#[derive(Debug)]
pub struct Dealer {
    pub cards: Vec<Card>,
}

pub trait Individual {
    fn set_card(&mut self, card: Card);
    fn sum(&self) -> u8;
}

impl Individual for Player {
    fn set_card(&mut self, card: Card) {
        self.cards.push(card);
    }

    fn sum(&self) -> u8 {
        self.cards.iter().map(|s| s.value).sum()
    }
}
impl Player {
    pub fn new(name: String) -> Self {
        Self { 
            cards: Vec::new(),
            name, 
        }
    }
}


impl Individual for Dealer {

    fn set_card(&mut self, card: Card) {
        self.cards.push(card);
    }

    fn sum(&self) -> u8 {
        self.cards.iter().map(|s| s.value).sum()
    }
}
impl Dealer {
    pub fn new() -> Self {
        Self { cards: Vec::new() }
    }
}