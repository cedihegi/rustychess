use crate::models::game_state::GameState;
use std::io;

use super::game::Game;

pub struct Driver {}

impl Driver {
    pub fn run_game() {
        let mut game = Game::new();
        loop {
            println!("It's {:?}'s turn", game.board.turn_color());
            println!("{}", game.board.to_pretty_string());

            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line");
            let outcome = game.apply_input(&input.trim());
            match outcome {
                Err(s) => println!("Move failed with error: {}", s),
                Ok(GameState::Ongoing) => println!("Move executed"),
                Ok(GameState::Won(color)) => {
                    println!("Color {color:?} has won!");
                    break;
                }
                Ok(GameState::Stalemate) => {
                    println!("Game ended in stalemate!!!");
                }
            }
        }
    }
}
