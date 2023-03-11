use std::u8;

use rand::Rng;

#[derive(Debug, PartialEq, Copy, Clone)]
/// Enum holding information about the type of a field.
pub enum Faction {
    /// Field is blocked., i.e. can't be occupied.
    Blocked,
    /// ID of the Player occupying the field.
    Player { id: u8 },
}

impl Faction {
    /// Creates a new faction object.
    ///
    /// Actual faction is randomized.
    ///
    /// * `max_players` - How many distinct player IDs are there.
    pub fn new(max_players: u8) -> Faction {
        let mut rng = rand::thread_rng();
        /* FIXME: have percentage of blocked fields be configurable */
        let rng_result = rng.gen_range(std::ops::Range{start: 0, end:
                                                                        if max_players == 255 {
                                                                            max_players
                                                                        } else {
                                                                            max_players + 1
                                                                        }});

        if max_players >= 255 {
            if rng.gen_range(0..256) == 0 {
                Faction::Blocked
            } else {
                Faction::Player { id: rng_result }
            }
        } else {
            if rng_result == 0 {
                Faction::Blocked
            } else {
                Faction::Player { id: rng_result - 1 }
            }
        }
    }

    pub fn r(&self) -> u8 {
        match self {
            Faction::Blocked => 0,
            Faction::Player { id } => (((id * 25) % 255) >> 5) * 32
        }
    }
    pub fn g(&self) -> u8 {
        match self {
            Faction::Blocked => 0,
            Faction::Player { id } => ((((id * 25) % 255) & 28) >> 2) * 32
        }
    }
    pub fn b(&self) -> u8 {
        match self {
            Faction::Blocked => 0,
            Faction::Player { id } => (((id * 25) % 255) & 3) * 64
        }
    }

    
}

impl Default for Faction {
    fn default() -> Faction {
        Faction::Blocked
    }
}