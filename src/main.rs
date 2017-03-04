extern crate dicewars;

use dicewars::{Game, Point};

fn main() {
    println!("Welcome to DiceWars!");
    let game: Game = Game::new(8, 6, Point { x: 10, y: 6 });

    game.print();
}
