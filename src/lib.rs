extern crate rand;


use rand::Rng;

/// Enum holding information about the type of a field.
pub enum Faction {
    /// Field is blocked., i.e. can't be occupied.
    Blocked,
    /// ID of the Player occupying the field.
    Player{ id: u8},
}

impl Faction {
    pub fn new(max_players: u8) -> Faction {
        let rng_result = rand::thread_rng().gen_range(0, if max_players == 255 {max_players} else { max_players + 1 });
        
        if max_players >= 255 {
            if rand::thread_rng().gen_range(0, 256) == 0 {
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
}

/// Information about a single field of the map.
pub struct Field {
    /// Stores the ID of the faction currently occupying this field.
    faction: Faction,
    
    /// Stores the number of dices on this field.
    num_dices: u8,
}

impl Field {
    pub fn new(max_players: u8, max_dices: u8) -> Field {
        Field { faction: Faction::new(max_players), num_dices: rand::thread_rng().gen_range(0, max_dices+1)}
    }
}

/// Describes the shape of the fields, i.e. how many direct neighbours a field has 
pub enum Shape {
    Square,
    //Hexagon,
}
/// Struct holding the map information
pub struct Map {
    /// How many fields along the X-Axis
    x_size: u8,
    
    /// How many fields along the Y-Axis
    y_size: u8,
    
    /// The actual map of the game.
    fields: Vec<Vec<Field>>,
}

impl Map {
    pub fn new(max_players: u8, max_dices: u8, x_size: u8, y_size: u8) -> Map {
        assert_ne!(max_players, 0);
        assert_ne!(max_dices, 0);
        assert_ne!(x_size, 0);
        assert_ne!(y_size, 0);

        let mut fields:Vec<Vec<Field>> = Vec::new();
        for _ in 0..x_size-1 {
            let mut column:Vec<Field> = Vec::new();
            for _ in 0..y_size-1 {
                column.push(Field::new(max_players, max_dices));
            }
            fields.push(column);
        }
        
        Map { x_size: x_size, y_size: y_size, fields:fields }
    }
}

/// Struct holding information about the Game itself
pub struct Game {
    /// How many players are part of the game
    num_players: u8,
    
    /// The maximal number of dices per field
    max_dices: u8,
    
    /// Stores the map of the game
    map:Map,
    
    /// Which round is it currently
    round: usize,
    
    /// whose turn it currently is
    turn: Faction,
}

impl Game {
    pub fn new(max_players: u8, max_dices: u8, x_size: u8, y_size: u8) -> Game {
        Game {
            num_players: max_players,
            max_dices: max_dices,
            map: Map::new(max_players, max_dices, x_size, y_size),
            round: 0,
            turn: Faction::Blocked
        }
    }
    
    pub fn get_dices_of_first_field(&self) -> u8 {
        self.map.fields[0][0].num_dices
    }
}