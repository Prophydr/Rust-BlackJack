mod card;
use card::{Card, Suits};


// use core::str;
// use std::string::ToString;
use strum::IntoEnumIterator;
// use strum_macros::EnumIter; 


fn main() {
    let suit_size = 14;

    let mut deck: Vec<Card> = Vec::new();

    for suit in Suits::iter() {

        for i in 1 .. suit_size {
            deck.push(Card::new(suit, i));
        }
    }


    for card in deck  {
        println!("Card: {0}", card.name);
        println!("Suit: {0}", card.suit);
        println!("Value: {0}", card.number);
        println!("Unicode: {0}", card.unicode);
    }
        
}
