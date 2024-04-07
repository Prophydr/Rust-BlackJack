mod card;
mod individual;
mod shoe;
use core::fmt;
use std::io;

use shoe::Shoe;

use individual::{Dealer, Player};

use self::individual::Individual;

// use self::individual::Individual;

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
        Self::help();
        loop {
        
            self.dealer.set_card(self.shoe.get_last_card(true));
            self.dealer.set_card(self.shoe.get_last_card(false));
            
            
            for player in self.players.iter_mut() {
                
                player.set_card(self.shoe.get_last_card(true));
                player.set_card(self.shoe.get_last_card(true));
            }

            
            let mut start = 0;
            let mut end = self.players.len()-1;
            println!("{:width$}", self.dealer, width = (end-start)*2);
            println!("");
            println!("");
            
            while start < end {
                
                // print!("{}," "\t".repeat(start*1));
                print!("{}", format!("{:width$}", self.players[start], width = (end-start)*1));
                print!("{}", format!("{:width$}", self.players[end], width = (end-start)*1));
                // println!(format!("{}{}{}", self.players[start], "\t".repeat((end-start)*1), self.players[end]));
                start += 1;
                end -= 1;
            }



            // for (i, player) in self.players.iter_mut().enumerate() {
                
            //     print!("{}{}", player, "\t\t".repeat(i*1));
            //     println!("");
            // }

            Self::user_input(&self);
        }
    }

    fn user_input(&self) -> bool {
        loop {
            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line");

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
