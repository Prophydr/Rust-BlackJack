use strum_macros::EnumIter;

#[derive(Debug, Clone)]
pub struct Card {
    pub suit: Suits,
    pub value: u8,
    pub name: String,
    pub unicode: char,
    pub back: char,
    pub face_up: bool,
}

impl Card {
    pub fn new(suit: Suits, value: u8) -> Self {
        Self {
            suit,
            value: Card::get_value(value),
            name: Card::get_name_from_value(value),
            unicode: Card::get_unicode(suit, value),
            back: char::from_u32(u32::from_str_radix("1F0A0", 16).expect("shit fuck")).expect("double fucked"),
            face_up: false,
        }
    }
    fn get_name_from_value(value: u8) -> String {
        match value {
            1 => "Ace".to_string(),
            2..=10 => value.to_string(),
            11 => "Jack".to_string(),
            12 => "Queen".to_string(),
            13 => "King".to_string(),
            _ => panic!("Unexpected value"),
        }
    }
    fn get_value(value: u8) -> u8 {
        match value {
            1..=10 => value,
            11..=13 => 10,
            _ => panic!("Unexpected number"),
        }
    }
    fn get_unicode(suit: Suits, value: u8) -> char {
        match suit {
            Suits::Clubs => Card::convert_to_char("D", value),
            Suits::Diamonds => Card::convert_to_char("C", value),
            Suits::Hearts => Card::convert_to_char("B", value),
            Suits::Spades => Card::convert_to_char("A", value),
        }
    }

    fn convert_to_char(mid_value: &str, value: u8) -> char {
        let head = "0x1F0";

        let end = match value {
            1..=9 => value.to_string(),
            10 => 'A'.to_string(), //10
            11 => 'B'.to_string(), //J
            12 => 'D'.to_string(), //Q
            13 => 'E'.to_string(), //K
            // 14 => 'E'.to_string(), //skip C becouse of tarot knight
            _ => panic!("Outside max size"),
        };

        let hex_str = u32::from_str_radix(&format!("{}{}{}", head, mid_value, end)[2..], 16)
            .expect("Can't convert to hex");
        return char::from_u32(hex_str).expect("Can't convert to char from u32");
    }
}

#[derive(Debug, EnumIter, strum_macros::Display, Clone, Copy)]
pub enum Suits {
    #[strum(to_string = "♣")]
    Clubs,
    #[strum(to_string = "♦")]
    Diamonds,
    #[strum(to_string = "♥")]
    Hearts,
    #[strum(to_string = "♠")]
    Spades,
}
