use super::card::Card;

pub struct Player {
    pub cards: Vec<Card>,
}

pub struct Dealer {
    pub cards: Vec<Card>,
}

trait Individual {
    fn new() -> Self;
    fn set_card(&self);
    fn sum(&self) -> u8;
}

impl Individual for Player {
    fn new() -> Self {
        Self { cards: Vec::new() }
    }

    fn set_card(&self) {
        todo!()
    }

    fn sum(&self) -> u8 {
        todo!()
    }
}

impl Individual for Dealer {
    fn new() -> Self {
        Self { cards: Vec::new() }
    }

    fn set_card(&self) {
        todo!()
    }

    fn sum(&self) -> u8 {
        todo!()
    }
}
