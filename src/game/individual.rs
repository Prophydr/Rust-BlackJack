use std::fmt::{self, Alignment};

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
impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(width) = f.width() {
            // If we received a width, we use it
            write!(f, "{}{}\n{}{})", "\t".repeat(width), String::from_iter(self.cards.iter().map(|f| f.unicode)), "\t".repeat(width), self.sum())
        } else {
            // Otherwise we do nothing special
            write!(f, "{}\n{})", String::from_iter(self.cards.iter().map(|f| f.unicode)), self.sum())
        }
    }
}


impl Individual for Dealer {

    fn set_card(&mut self, card: Card) {
        self.cards.push(card);
    }

    fn sum(&self) -> u8 {
        self.cards.iter().filter(|c| c.face_up).map(|c| c.value).sum()
    }
}
impl Dealer {
    pub fn new() -> Self {
        Self { cards: Vec::new() }
    }
}
impl fmt::Display for Dealer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut cards: String = "".to_string();

        for c in self.cards.iter() {
            if c.face_up {
                cards.push(c.unicode);
            }
            else {
                cards.push(c.back)
            }
        }

        if let Some(width) = f.width() {
            // If we received a width, we use it
            write!(f, "{}{}\n{}{}", "\t".repeat(width), cards, "\t".repeat(width), self.sum())
        } else {
            // Otherwise we do nothing special
            write!(f, "{}\nt{}", cards, self.sum())
        }
    }
}