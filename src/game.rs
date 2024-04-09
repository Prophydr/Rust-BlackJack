mod card;
mod individual;
mod shoe;

use std::io;

use shoe::Shoe;

use individual::{Dealer, Player};

use self::individual::Individual;

#[derive(Debug)]
pub struct Game {
    shoe: Shoe,
    dealer: Dealer,
    players: Vec<Player>,
}

impl Game {
    pub fn new(number_of_decks: u8, players: Vec<String>, shuffle: bool) -> Self {
        Self {
            shoe: Shoe::new(number_of_decks, shuffle),
            dealer: Dealer::new(),
            players: players
                .iter()
                .map(|name| Player::new(name.clone()))
                .collect(),
        }
    }

    pub fn start(&mut self) {
        self.dealer.set_card(self.shoe.get_last_card(true));
        self.dealer.set_card(self.shoe.get_last_card(false));

        for player in self.players.iter_mut() {
            player.set_card(self.shoe.get_last_card(true));
            player.set_card(self.shoe.get_last_card(true));
        }

        loop {
            print!("{}[2J", 27 as char);

            self.display();

            println!("");
            println!("");
            Self::help();
            Self::user_input(&self);
        }
    }

    fn display(&mut self) {
        let mut start = 0;
        let mut end = self.players.len() - 1;
        let mut start_number_t = self.players.len();
        let mut end_number_t = self.players.len() * 2;

        println!("{}{}", "\t".repeat(self.players.len() * 2), self.dealer);
        println!(
            "{}{}",
            "\t".repeat(self.players.len() * 2),
            self.dealer.sum()
        );
        println!("");

        while start < end {
            println!("");

            print!("{}{}", "\t".repeat(start_number_t), self.players[start]);

            if start != end {
                print!("{}{}", "\t".repeat(end_number_t), self.players[end],);
            }

            println!("");

            print!(
                "{}{}",
                "\t".repeat(start_number_t),
                self.players[start].sum()
            );

            if start != end {
                print!("{}{}", "\t".repeat(end_number_t), self.players[end].sum());
            }

            start_number_t += 2;
            end_number_t = end_number_t.wrapping_sub(4);

            start += 1;
            end -= 1;
        }

        println!("");

        if self.players.len() % 2 != 0 {
            print!(
                "{}{}",
                "\t".repeat(self.players.len() * 2),
                self.players[start]
            );

            println!("");

            print!(
                "{}{}",
                "\t".repeat(self.players.len() * 2),
                self.players[start].sum()
            );
        }
    }

    fn user_input(&self) -> bool {
        loop {
            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line");

            // Remove trailing newline character
            input = input.trim().to_string();

            match input.as_str() {
                "!q" => std::process::exit(0),
                "h" => return true,
                "s" => return false,
                _ => println!("Does not reqognice the input try again"),
            };
        }
    }

    fn help() {
        println!("!q to quit / h to hit / s to stand");
    }
}
