extern crate dicewars;

use dicewars::Game;

fn main() {
    println!("Welcome to DiceWars!");
    let game:Game = Game::new(2, 6, 6, 4);
    
    println!("number of dices on the first fields: {}",game.get_dices_of_first_field());
    game.print();
}