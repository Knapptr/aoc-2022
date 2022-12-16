use core::fmt;
use std::{
    fmt::{write, Display, Formatter},
    panic,
};

const SAND_ORIGIN_X: usize = 500;

#[derive(Debug, Clone, Copy)]
enum Cell {
    Empty,
    Rock,
    Void,
    Sand,
}

#[derive(Debug, Clone, Copy, Eq)]
pub struct Coords {
    x: usize,
    y: usize,
}
impl PartialEq for Coords {
    fn eq(&self, rhs: &Self) -> bool {
        self.x == rhs.x && self.y == rhs.y
    }
}
impl Coords {
    fn at(x: usize, y: usize) -> Self {
        Self { x, y }
    }
    fn is_at_bounds(&self, grid: &RockGrid) -> bool {
        if self.x == 0 {
            return true;
        }
        if self.y == grid.height() as usize - 1 {
            return true;
        }
        if self.x == grid.width() as usize - 1 {
            return true;
        }
        false
    }
    fn at_descend(&self) -> Coords {
        Coords {
            x: self.x,
            y: self.y + 1,
        }
    }
    fn at_desc_left(&self) -> Coords {
        Coords {
            x: self.x - 1,
            y: self.y + 1,
        }
    }
    fn at_desc_right(&self) -> Coords {
        Coords {
            x: self.x + 1,
            y: self.y + 1,
        }
    }
}
#[derive(Debug)]
pub struct RockGrid {
    grid: Vec<Vec<Cell>>,
    min_x: usize,
    pub drop_possible: bool,
}

impl RockGrid {
    pub fn from_input(input: &str) -> Self {
        RockGrid::from_rock_commands(RockCommand::vec_from_input(input))
    }
    fn sand_origin(&self) -> usize {
        SAND_ORIGIN_X - self.min_x
    }
    fn height(&self) -> isize {
        self.grid.len() as isize
    }
    fn width(&self) -> isize {
        self.grid[0].len() as isize
    }
    pub fn drop_sand_grain(&mut self) {
        let origin = self.sand_origin();
        // self.grid[0][origin] = Cell::Sand;
        let mut current_coord: Coords = Coords::at(self.sand_origin(), 0);
        while !current_coord.is_at_bounds(&self) {
            match self.get_at_coords(&current_coord.at_descend()) {
                Cell::Empty => {
                    current_coord = current_coord.at_descend();
                    continue;
                }
                _ => {
                    //check left
                    match self.get_at_coords(&current_coord.at_desc_left()) {
                        Cell::Empty => {
                            current_coord = current_coord.at_desc_left();
                            continue;
                        }
                        _ => (),
                    }
                    //check right
                    match self.get_at_coords(&current_coord.at_desc_right()) {
                        Cell::Empty => {
                            current_coord = current_coord.at_desc_right();
                            continue;
                        }
                        _ => {
                            *self.get_at_coords_mut(&current_coord) = Cell::Sand;
                            break;
                        }
                    }
                }
            }
        }
        let mut row_string = String::new();
        for cell in &self.grid[current_coord.y] {
            match cell {
                Cell::Rock => row_string.push_str("X"),
                Cell::Empty => row_string.push_str("."),
                _ => (),
            }
        }
        if current_coord.is_at_bounds(self) {
            self.drop_possible = false;
        }
    }
    fn get_at_coords_mut(&mut self, coords: &Coords) -> &mut Cell {
        &mut self.grid[coords.y][coords.x]
    }
    fn get_at_coords(&self, coords: &Coords) -> &Cell {
        &self.grid[coords.y][coords.x]
    }
    fn from_rock_commands(commands: Vec<RockCommand>) -> Self {
        // this is bad. Redo this bit, needless iteration.
        let min_x = commands
            .iter()
            .map(|rc| rc.get_all_coords().iter().map(|c| c.x).min().unwrap())
            .min()
            .unwrap();
        let max_x = commands
            .iter()
            .map(|rc| rc.get_all_coords().iter().map(|c| c.x).max().unwrap())
            .max()
            .unwrap();
        let max_y = commands
            .iter()
            .map(|rc| rc.get_all_coords().iter().map(|c| c.y).max().unwrap())
            .max()
            .unwrap();

        // subtract min x from all x coords
        let mut grid = vec![vec![Cell::Empty; (max_x - min_x + 1) as usize]; max_y as usize];
        // let mut grid = vec![vec![Cell::Empty; (max_x  + 1) as usize]; max_y as usize];

        for command in commands {
            let all_coords = command.get_all_coords();
            for coord in all_coords {
                grid[(coord.y - 1) as usize][(coord.x - min_x) as usize] = Cell::Rock
                // grid[(coord.y - 1) as usize][(coord.x ) as usize] = Cell::Rock
            }
        }
        RockGrid {
            min_x,
            grid,
            drop_possible: true,
        }
    }
}
impl Display for RockGrid {
    // Renders from Top to bottom
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let mut str = String::new();
        for row in &self.grid {
            for cell in row {
                match cell {
                    Cell::Rock => str.push_str("X"),
                    Cell::Empty => str.push_str("."),
                    Cell::Sand => str.push_str("$"),
                    _ => str.push_str("_"),
                }
            }
            str.push_str("\n")
        }
        write!(f, "{}", str)
    }
}

struct RockCommand {
    coords: Vec<Coords>,
}
impl RockCommand {
    fn init() -> Self {
        Self { coords: Vec::new() }
    }
    fn vec_from_input(input: &str) -> Vec<Self> {
        let mut vec = Vec::new();
        for line in input.lines() {
            vec.push(RockCommand::from_input(line))
        }
        vec
    }
    fn from_input(input: &str) -> Self {
        let mut rock_command = RockCommand::init();
        let command_strs = input.split(" -> ");

        for command_str in command_strs {
            let (x_coord, y_coord) = command_str
                .split_once(",")
                .map(|(x, y)| {
                    (
                        x.parse::<usize>().expect("cannot parse"),
                        y.parse::<usize>().expect("cannot parse"),
                    )
                })
                .expect("Bad str");
            rock_command.coords.push(Coords::at(x_coord, y_coord));
        }
        rock_command
    }
    // 498,4 -> 498,6 -> 496,6
    fn get_all_coords(&self) -> Vec<Coords> {
        let mut idx = 0;
        let mut all_coords = Vec::new();
        for idx in 0..self.coords.len() - 1 {
            let left = self.coords[idx];
            let right = self.coords[idx + 1];

            if left.y > right.y {
                // Going down
                for y in 0..left.y - right.y {
                    all_coords.push(Coords::at(left.x, left.y - y));
                }
            }
            if right.y > left.y {
                //going up
                for y in 0..right.y - left.y {
                    all_coords.push(Coords::at(left.x, left.y + y));
                }
            }
            if left.x > right.x {
                for x in 0..left.x - right.x {
                    all_coords.push(Coords::at(left.x - x, left.y))
                }
            }
            if right.x > left.x {
                for x in 0..right.x - left.x {
                    all_coords.push(Coords::at(left.x + x, left.y))
                }
            }
        }
        all_coords.push(*self.coords.last().unwrap());
        all_coords
    }
}
#[cfg(test)]
#[test]
fn rocks_to_grid() {
    let test_in = "498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9";
    let rock_commands = RockCommand::vec_from_input(test_in);
    for rock_command in &rock_commands {}
    let rock_grid = RockGrid::from_rock_commands(rock_commands);
    assert_eq!(rock_grid.grid.len(), 9);
    assert_eq!(rock_grid.grid[0].len(), 10);
}
#[test]
fn parse_rock_input() {
    let test_in = "498,4 -> 498,6 -> 496,6";
    let rock_command = RockCommand::from_input(test_in);
    assert_eq!(rock_command.coords[0], Coords { x: 498, y: 4 })
}
#[test]
fn rock_all_coords() {
    let test_in = "498,4 -> 498,6 -> 496,6";
    let rock_command = RockCommand::from_input(test_in);
    assert_eq!(
        rock_command.get_all_coords(),
        vec![
            Coords::at(498, 4),
            Coords::at(498, 5),
            Coords::at(498, 6),
            Coords::at(497, 6),
            Coords::at(496, 6),
        ]
    );
    assert_eq!(rock_command.get_all_coords().len(), 5)
}
#[test]
fn sand_x() {
    let test_in = "498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9";
    let rock_commands = RockCommand::vec_from_input(test_in);
    let grid = RockGrid::from_rock_commands(rock_commands);
    assert_eq!(grid.sand_origin(), 6);
}
#[test]
fn rock_all_coords_2() {
    let test_in = "503,4 -> 502,4 -> 502,9 -> 494,9";
    let rock_command = RockCommand::from_input(test_in);
    assert_eq!(rock_command.get_all_coords().len(), 15)
}
