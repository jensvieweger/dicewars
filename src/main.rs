extern crate dicewars;

use dicewars::Game;

fn main() {
    println!("Welcome to DiceWars!");
    let game: Game = Game::new(8, 6, 10, 6);

    game.print();
}
