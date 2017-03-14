//! Everything related to game error handling
pub use std::error::Error;
use std::fmt;

/// The global Error type for the game
pub struct GameError {
    /// The type of the thrown error
    pub code: ErrorType,

    /// A further description for the error
    pub description: String,
}

/// Representation of an error case
impl GameError {
    /// Creates a new `GameError`
    pub fn new(code: ErrorType, description: &str) -> Self {
        GameError {
            code: code,
            description: description.to_string(),
        }
    }
}

impl fmt::Display for GameError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{:?}] {}", self.code, self.description)
    }
}

impl fmt::Debug for GameError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}

impl Error for GameError {
    fn description(&self) -> &str {
        &self.description
    }
}

/// Common Game Result type
pub type GameResult<T> = Result<T, GameError>;

#[derive(Debug, PartialEq)]
/// Error codes as indicator what happened
pub enum ErrorType {
    /// Everything worked fine
    Ok,

    /// Selected field can not be an attacker
    InvalidAttacker,

    /// Selected field can not be attacked
    InvalidTarget,

    /// An unspecific error occured
    Other,
}
