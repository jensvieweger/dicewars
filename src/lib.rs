extern crate rand;
extern crate colored;

mod error;
pub use error::{ErrorType, GameResult, GameError};

mod faction;
mod map;
pub use map::Point;

mod game;
pub use game::Game;