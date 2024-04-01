use super::card::{Card, Suits};
use rand::seq::SliceRandom;
use rand::thread_rng;
use strum::IntoEnumIterator;

pub struct Shoe {
    pub number_of_decks: u8,
    pub decks: Vec<Card>,
    pub discard: Vec<Card>,
}

impl Shoe {
    pub fn new(number_of_decks: u8, shuffle: bool) -> Self {
        Self {
            number_of_decks,
            decks: Self::create_decks(number_of_decks, shuffle),
            discard: Vec::new(),
        }
    }

    fn create_deck() -> Vec<Card> {
        let suit_size = 14;
        let mut deck: Vec<Card> = Vec::new();

        for suit in Suits::iter() {
            for i in 1..suit_size {
                deck.push(Card::new(suit, i));
            }
        }

        return deck;
    }

    fn create_decks(number_of_decks: u8, shuffle: bool) -> Vec<Card> {
        let mut decks: Vec<Card> = Vec::new();

        for _ in 0..number_of_decks {
            decks.append(&mut Self::create_deck());
        }

        if shuffle {
            decks.shuffle(&mut thread_rng())
        }

        return decks;
    }

    pub fn shuffle(&mut self) {
        self.decks.shuffle(&mut thread_rng());
    }

    pub fn get_last_card(&mut self) -> Card {
        let card = self.decks.pop();

        match card {
            Some(c) => {
                self.discard.push(c.clone());
                return c;
            }
            None => {
                self.decks.append(&mut self.discard);

                self.shuffle();

                return Self::get_last_card(self);
            }
        }
    }
}
