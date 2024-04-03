use std::io;

use game::Game;

mod game;

fn main() {

    let number_of_decks = get_user_num_data("number of decks: ".to_owned());
    let number_of_players = get_user_num_data("number of players: ".to_owned());
    let mut players: Vec<String> = Vec::new();

    for index in 0..number_of_players {
        players.push(get_user_string_data(format!("Name of player {}: ", index+1)));
    }

    let game: Game = Game::new(number_of_decks, players, true );


    // for i in 0..shoe.decks.len() + 1 {
    //     let card = shoe.get_last_card();
    //     print!("Card: {0} ", card.name);
    //     print!("Suit: {0} ", card.suit);
    //     print!("Value: {0} ", card.number);
    //     println!("Unicode: {0} ", card.unicode);
    // }
}

fn get_user_string_data(print_string: String) -> String {
    
    print!("{}", print_string);
    io::Write::flush(&mut io::stdout()).unwrap(); // Flush stdout to ensure prompt is displayed immediately

    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    input
}


fn get_user_num_data(print_string: String) -> u8 {
    
    print!("{}", print_string);
    io::Write::flush(&mut io::stdout()).unwrap(); // Flush stdout to ensure prompt is displayed immediately

    let mut input = String::new();
    io::stdin().read_line(&mut input)
        .expect("Failed to read line");

    let mut number: u8 = 0;
    while number == 0 {
        // Parse the input into a u8
        number = match input.trim().parse() {
            Ok(num) => {
                num
            },
            Err(_) => {
                println!("Invalid input. Please enter a number between 1 and 255.");
                0
            }
        };
    }
    return  number;
}