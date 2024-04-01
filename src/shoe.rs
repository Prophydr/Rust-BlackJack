mod card;

pub mod shoe {
    use super::card::card::Card;

    pub struct Shoe {
        number_of_decks: u8,
        decks: Vec<Card>,
    }

    impl Shoe {
        pub fn new(number_of_decks: u8) -> Self {
            Self {
                number_of_decks,
                decks: Self::create_decks(number_of_decks),
            }
        }
        fn create_decks(number_of_decks: u8) -> Vec<Card> {
            todo!()
        }
    }
}
