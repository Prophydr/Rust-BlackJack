mod shoe;
use shoe::shoe::Shoe;
// use shoe::shoe::Shoe;

// mod card;
// mod shoe;
// use card::{Card, Suits};
// mod card;
// use shoe::shoe::Shoe;


// use core::str;
// use std::string::ToString;
// use strum::IntoEnumIterator;
// use strum_macros::EnumIter; 


fn main() {

    let mut shoe: Shoe = Shoe::new(1, false);

    // let suit_size = 14;

    // let mut deck: Vec<Card> = Vec::new();

    // for suit in Suits::iter() {

    //     for i in 1 .. suit_size {
    //         deck.push(Card::new(suit, i));
    //     }
    // }


    for i in 0..shoe.decks.len() + 1 {
        let card = shoe.get_last_card();
        print!("Card: {0} ", card.name);
        print!("Suit: {0} ", card.suit);
        print!("Value: {0} ", card.number);
        println!("Unicode: {0} ", card.unicode);
    }

    
        
}
