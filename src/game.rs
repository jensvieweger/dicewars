use std::u8;

use rand::Rng;
use colored::*;

use faction::Faction;
use map::*;
use {ErrorType, GameResult, GameError};

/// Struct holding information about the Game itself
pub struct Game {
    /// How many players are part of the game
    num_players: u8,

    /// The maximal number of dices per field
    max_dice: u8,

    /// Stores the map of the game
    map: Map,

    /// Which round is it currently
    round: usize,

    /// whose turn it currently is
    turn: Faction,
}

impl Game {
    pub fn new(max_players: u8, max_dice: u8, size: Point) -> Game {
        Game {
            num_players: max_players,
            max_dice: max_dice,
            map: Map::new(max_players, max_dice, size),
            round: 0,
            turn: Faction::Player { id: 1 },
        }
    }

    pub fn print(&self) {
        match self.turn {
            Faction::Blocked => panic!("the turn must be on a valid faction"),
            Faction::Player { id } => {
                println!("Player: {}/{}, round: {}",
                         id.to_string().truecolor(self.turn.r(), self.turn.g(),self.turn.b()),
                         self.num_players,
                         self.round);
            }
        }
        self.map.pretty_print();
    }

    pub fn turn(&mut self, atk_from: Point, atk_to: Point) -> GameResult<()> {
        let attacker = self.map.get_field(atk_from);
        let target = self.map.get_field(atk_to);

        if attacker.num_dice <= 1 {
            return Err(GameError::new(ErrorType::InvalidAttacker,
                                      "Attacker must have more than 1 dice."));
        }
        match target.faction {
            y if y == attacker.faction => {
                return Err(GameError::new(ErrorType::InvalidTarget,
                                          "Target is of the same faction as the attacker."))
            }
            Faction::Blocked => {
                return Err(GameError::new(ErrorType::InvalidTarget, "Can't attack blocked field."))
            }
            _ => {}
        }
        if attacker.faction == target.faction {
            return Err(GameError::new(ErrorType::InvalidTarget,
                                      "Target is of the same faction as the attacker."));
        }

        let attacker_strength = self.roll_dice(attacker.num_dice);
        let target_strength = self.roll_dice(target.num_dice);

        if attacker_strength > target_strength {
            /*target = Field {
                num_dice: attacker.num_dice - 1,
                faction: attacker.faction,
            };*/
            let mut target_mut = self.map.get_field_mut(atk_to);
            target_mut.num_dice = attacker.num_dice - 1;
            target_mut.faction = attacker.faction;
        }
        {
            let mut attacker_mut = self.map.get_field_mut(atk_from);
            attacker_mut.num_dice = 1;
        }
        Ok(())
    }

    pub fn finalize_turn(&mut self) {
        let mut row_iter = self.map.fields.iter_mut();
        let row = row_iter.next().expect("Invalid map (no row available)");

        loop {
            let mut field_iter = row.iter_mut();
            let mut field = field_iter.next().expect("Invalid map (row is empty)");

            loop {
                match field.faction {
                    Faction::Blocked => {},
                    Faction::Player { id} => {
                        if field.num_dice < self.max_dice {
                            field.num_dice += 1;
                        }
                    }
                };
            }
        }
    }

    fn roll_dice(&self, numdice: u8) -> usize {
        let mut result = 0;
        for _ in 0..numdice {
            result += rand::thread_rng().gen_range(std::ops::Range{start: 1, end: 6 + 1});
        }
        result
    }
}