use std::u8;

use rand::Rng;
use colored::*;
use faction::Faction;

#[derive(Debug, PartialEq, Copy, Clone)]
/// Information about a single field of the map.
pub struct Field {
    /// Stores the ID of the faction currently occupying this field.
    pub faction: Faction,

    /// Stores the number of dice on this field.
    pub num_dice: u8,
}

impl Field {
    pub fn new(max_players: u8, max_dice: u8) -> Field {
        Field {
            faction: Faction::new(max_players),
            num_dice: rand::thread_rng().gen_range(std::ops::Range{start: 1, end: max_dice + 1}),
        }
    }
}

impl Default for Field {
    fn default() -> Field {
        Field {
            faction: Faction::default(),
            num_dice: 0,
        }
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
/// Describes a point in 2D space
pub struct Point {
    /// The points location along the X-axis.
    pub x: u8,

    /// The points location along the Y-axis.
    pub y: u8,
}

/// Describes the shape of the fields, i.e. how many direct neighbours a field has
pub enum Shape {
    Square,
}

/// Struct holding the map information
pub struct Map {
    /// How many fields on each axis
    size: Point,

    /// The actual map of the game. use [y][x] indexing
    pub fields: Vec<Vec<Field>>,
}

impl Map {
    /// Initialises an empty map.
    ///
    /// Each field is blocked, with 0 dice on it.
    ///
    /// * `size` - Dimension of the map in number of fields per axis.
    pub fn new_empty(size: Point) -> Map {
        assert_ne!(size.x, 0);
        assert_ne!(size.y, 0);

        let mut fields: Vec<Vec<Field>> = Vec::new();
        for _ in 0..size.y {
            let mut row: Vec<Field> = Vec::new();
            for _ in 0..size.x {
                row.push(Field::default());
            }
            fields.push(row);
        }

        Map {
            size: size,
            fields: fields,
        }
    }

    /// Initialises a randomized map.
    ///
    /// Each field is randomly generated, resulting in a playable map
    ///
    /// * `players` - Number of player to distribute.
    /// * `max_dice` - Number of dice, a field may have at max.
    /// * `size` - Dimension of the map in number of fields per axis.
    pub fn new(players: u8, max_dice: u8, size: Point) -> Map {
        assert_ne!(players, 0);
        assert_ne!(max_dice, 0);
        assert_ne!(size.x, 0);
        assert_ne!(size.y, 0);

        let mut fields: Vec<Vec<Field>> = Vec::new();
        for _ in 0..size.y {
            let mut row: Vec<Field> = Vec::new();
            for _ in 0..size.x {
                row.push(Field::new(players, max_dice));
            }
            fields.push(row);
        }

        let mut map: Map = Map {
            size: size,
            fields: fields,
        };
        map.sanitize_map();
        map
    }

    /// after initial generation, this function makes sure that the generated map is actually playable
    /// here: all non-blocked fields must be connected and each player must have ~ the same amount of dice
    fn sanitize_map(&mut self) -> &mut Map {
        /* FIXME: implement */
        //unimplemented!();
        //self.fields[0][0].num_dice = 1;
        //println!("{} {}", self.fields.len(), self.fields[0].len());
        self
    }

    /// Retrieves the field at the given coordinate.
    ///
    /// Returns a field.
    ///
    /// * `coord` - Coordinate of the field to retrieve.
    pub fn get_field(&self, coord: Point) -> Field {
        assert!(coord.x < self.size.x);
        assert!(coord.y < self.size.y);
        self.fields[coord.y as usize][coord.x as usize]
    }

    /// Retrieves the field at the given coordinate.
    ///
    /// Returns a mutable reference to the field.
    ///
    /// * `coord` - Coordinate of the field to retrieve.
    pub fn get_field_mut(&mut self, coord: Point) -> &mut Field {
        assert!(coord.x < self.size.x);
        assert!(coord.y < self.size.y);
        &mut self.fields[coord.y as usize][coord.x as usize]
    }

    /// Prints the map to stdout, which should then look like this:
    /// ╔═╤═╤═╤═╤═╗
    /// ║1│2│5│-│4║
    /// ╟─┼─┼─┼─┼─╢
    /// ║2│4│6│-│-║
    /// ╚═╧═╧═╧═╧═╝
    ///
    pub fn pretty_print(&self) {
        // header
        print!("  ");
        for n_col in 0..self.fields[0].len() {
            print!("{} ", n_col.to_string());
        }
        println!("");

        print!(" ╔");
        for _ in 0..self.fields[0].len() - 1 {
            print!("═╪");
        }
        println!("═╗");

        let mut row_iter = self.fields.iter().peekable();
        let mut row = row_iter.next().expect("Invalid map (no row available)");
        let mut n_row = 0;

        loop {
            print!("{}║", n_row.to_string());

            let mut field_iter = row.iter().peekable();
            let mut field = field_iter.next().expect("Invalid map (row is empty)");

            loop {
                match field.faction {
                    Faction::Blocked => print!("{}", "-".white().bold()),
                    Faction::Player { ..} => {
                        //print!("{} ({}) ", Green.paint(field.num_dice.to_string()), id)
                        // TODO: try to get some nice looking palette (as ansi_term's 'Fixed(u8)' did)
                        print!("{}", field.num_dice.to_string().truecolor(field.faction.r(), field.faction.g(),field.faction.b()))
                    }
                };
                match field_iter.peek() {
                    Some(_) => print!("│"),
                    None => {
                        println!("║");
                        break;
                    }
                }
                field = field_iter.next().expect("Invalid map (row is empty)");
            }

            match row_iter.peek() {
                Some(_) => {
                    // between the lines
                    print!("─╫");
                    for _ in 0..self.fields[0].len() - 1 {
                        print!("─┼");
                    }
                    println!("─╢");
                }
                None => break,
            }
            row = row_iter.next().expect("Invalid map (no row available)");
            n_row += 1;
        }

        // footer
        print!(" ╚");
        for _ in 0..self.fields[0].len() - 1 {
            print!("═╧");
        }
        println!("═╝");

    }
}