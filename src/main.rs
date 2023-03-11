extern crate dicewars;
extern crate termion;

use dicewars::{Game, Point};

use termion::clear;




use std::io::{self, Read, Write};

fn main() {
    let stdin = io::stdin();
    let stdin = stdin.lock();
    let stdout = io::stdout();
    let mut stdout = stdout.lock();
    //let mut stdout = stdout.into_raw_mode().unwrap();

    println!("Welcome to DiceWars!");
    let mut game: Game = Game::new(8, 6, Point { x: 10, y: 6 });

    game.print();

    let mut bytes = stdin.bytes();
    loop {
        // Read a single byte from stdin.
        let b = bytes.next().unwrap().unwrap();
        match b as char {
            '1' => {
                game.turn(Point { x: 0, y: 1 }, Point { x: 1, y: 1 });
            }
            '2' => {
                game.turn(Point { x: 0, y: 2 }, Point { x: 1, y: 2 });
            }
            '3' => {
                game.turn(Point { x: 0, y: 3 }, Point { x: 1, y: 3 });
            }
            '4' => {
                game.turn(Point { x: 0, y: 4 }, Point { x: 1, y: 4 });
            }
            '5' => {
                game.turn(Point { x: 0, y: 5 }, Point { x: 1, y: 5 });
            }
            'q' => return,
            _ => {}
        }
        write!(stdout, "{}", clear::All).unwrap();
        game.print();

        // tick(faction)
    }
}
