use core::fmt;
use std::{
    fmt::{write, Display, Formatter},
    panic,
};

const SAND_ORIGIN_X: usize = 500;
const PART_2_WIDTH: usize = 1000;

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
    part_1: bool,
    drop_count: u32,
    floor_width: Option<usize>,
}

impl RockGrid {
    pub fn from_input(input: &str, part_1: bool, floor_width: Option<usize>) -> Self {
        RockGrid::from_rock_commands(RockCommand::vec_from_input(input), part_1, floor_width)
    }
    fn sand_origin(&self) -> usize {
        if self.part_1 {
            SAND_ORIGIN_X - self.min_x
        } else {
            SAND_ORIGIN_X - self.min_x + self.floor_width.unwrap() / 2
        }
    }
    pub fn height(&self) -> isize {
        self.grid.len() as isize
    }
    pub fn width(&self) -> isize {
        self.grid[0].len() as isize
    }
    pub fn drop_sand_grain(&mut self) {
        //starting coord of drop
        let mut current_coord: Coords = Coords::at(self.sand_origin(), 0);
        // test to see if continued drop is possible
        while !current_coord.is_at_bounds(&self) {
            // println!("Drop is at: {:?}", current_coord);
            match self.get_at_coords(&current_coord) {
                Cell::Empty => {
                    // println!("Checking current location: {:?}", current_coord);
                    // check if possible to descend
                    match self.get_at_coords(&current_coord.at_descend()) {
                        Cell::Empty => {
                            current_coord = current_coord.at_descend();
                            continue;
                        }
                        _ => {
                            //check left
                            // println!("Cannot drop straight down");
                            match self.get_at_coords(&current_coord.at_desc_left()) {
                                Cell::Empty => {
                                    current_coord = current_coord.at_desc_left();
                                    continue;
                                }
                                _ => {
                                    // println!("Cannot drop left");
                                }
                            }
                            //check right
                            match self.get_at_coords(&current_coord.at_desc_right()) {
                                Cell::Empty => {
                                    current_coord = current_coord.at_desc_right();
                                    continue;
                                }
                                _ => {
                                    // println!("Placing sand");
                                    *self.get_at_coords_mut(&current_coord) = Cell::Sand;
                                    break;
                                }
                            }
                        }
                    }
                }
                _ => {
                    println!("Current cell not empty?")
                }
            }
        }
        // for part 2
        if current_coord == Coords::at(self.sand_origin(), 0) {
            self.drop_possible = false;
        }
        if current_coord.is_at_bounds(self) {
            *self.get_at_coords_mut(&current_coord) = Cell::Sand;
            self.drop_possible = false;
        }
    }
    fn get_at_coords_mut(&mut self, coords: &Coords) -> &mut Cell {
        &mut self.grid[coords.y][coords.x]
    }
    fn get_at_coords(&self, coords: &Coords) -> &Cell {
        &self.grid[coords.y][coords.x]
    }
    fn from_rock_commands(
        commands: Vec<RockCommand>,
        part_1: bool,
        floor_width: Option<usize>,
    ) -> Self {
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

        let mut grid;
        if part_1 {
            grid = vec![vec![Cell::Empty; (max_x - min_x + 1) as usize]; max_y + 2 as usize];
        } else {
            if let Some(floor_width) = floor_width {
                grid = vec![vec![Cell::Empty; (floor_width) as usize]; max_y + 3 as usize];
            } else {
                grid = vec![vec![Cell::Empty; (PART_2_WIDTH) as usize]; max_y + 3 as usize];
            }
        }
        // let mut grid = vec![vec![Cell::Empty; (max_x  + 1) as usize]; max_y as usize];

        for command in commands {
            let all_coords = command.get_all_coords();
            for coord in all_coords {
                if part_1 {
                    grid[coord.y][(coord.x - min_x) as usize] = Cell::Rock
                } else {
                    if let Some(floor_width) = floor_width {
                        grid[coord.y][coord.x - min_x + floor_width / 2] = Cell::Rock
                    } else {
                        grid[(coord.y - 1)][(coord.x)] = Cell::Rock
                    }
                }
                // grid[(coord.y - 1) as usize][(coord.x ) as usize] = Cell::Rock
            }
        }
        if !part_1 {
            for cell in &mut grid[max_y + 2] {
                *cell = Cell::Rock
            }
        }
        RockGrid {
            min_x,
            grid,
            drop_possible: true,
            part_1,
            drop_count: 0,
            floor_width,
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
    let grid = RockGrid::from_rock_commands(rock_commands, false, None);
    assert_eq!(grid.sand_origin(), 6);
}
#[test]
fn rock_all_coords_2() {
    let test_in = "503,4 -> 502,4 -> 502,9 -> 494,9";
    let rock_command = RockCommand::from_input(test_in);
    assert_eq!(rock_command.get_all_coords().len(), 15)
}
