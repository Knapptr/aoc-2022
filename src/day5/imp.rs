use std::{fmt::Error, str::FromStr};

// Ship

#[derive(Debug)]
pub struct Ship {
    crates: Vec<Vec<char>>,
}
impl Ship {
    pub fn from_input(stack_input: &str) -> Ship {
        let mut ship_crates: Vec<Vec<char>> = Vec::new();
        for line in stack_input.lines().rev().skip(1) {
            for (idx, char) in line.chars().skip(1).step_by(4).enumerate() {
                if (ship_crates.len()) <= idx {
                    ship_crates.push(Vec::new())
                }
                if char.is_alphabetic() {
                    ship_crates[idx].push(char)
                }
            }
        }
        Ship {
            crates: ship_crates,
        }
    }
    pub fn top_crates(&self) -> String {
        let mut top_crates = String::new();
        for stack in &self.crates {
            let top_crate = stack.last().expect("No top crate").clone();
            top_crates.push(top_crate);
        }
        top_crates
    }
    pub fn move_9000(&mut self, command: Command) {
        for _x in 0..command.quantity {
            let removed_crate = self.crates[command.from].pop().expect("wont fail");
            self.crates[command.to].push(removed_crate);
        }
    }
    pub fn move_9001(&mut self, command: Command) {
        let removal_stack = &mut self.crates[command.from];
        let mut removed_items: Vec<char> = removal_stack
            .drain(removal_stack.len() - command.quantity..)
            .collect();
        self.crates[command.to].append(&mut removed_items);
    }
}

// Commands

#[derive(Debug)]
pub struct Commands {
    pub list: Vec<Command>,
}
impl Commands {
    pub fn from_input(input: &str) -> Commands {
        let mut command_list = Vec::new();
        for line in input.lines() {
            command_list.push(Command::from_str(line).expect("data is uniform"))
        }
        Commands { list: command_list }
    }
}

#[derive(Debug)]
pub struct Command {
    quantity: usize,
    from: usize,
    to: usize,
}
impl FromStr for Command {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let nums: Vec<usize> = s
            .split_whitespace()
            .filter_map(|string| string.parse().ok())
            .collect();
        Ok(Command {
            quantity: nums[0],
            from: nums[1] - 1,
            to: nums[2] - 1,
        })
    }
}

#[cfg(test)]
#[test]
fn parses_ship_struct() {
    let test_input = "[D] [A] [T]\n[X] [V] [B]\n 1   2   3";
    let mut ship = Ship::from_input(test_input);
    assert_eq!(ship.crates.len(), 3);
    assert_eq!(ship.crates[1].len(), 2);
    assert_eq!(ship.crates[1].pop().unwrap(), 'A');
}
#[test]
fn command_parse() {
    let input = "move 1 from 7 to 8";
    let command = Command::from_str(input).expect("Problem making command");
    assert_eq!(command.quantity, 1);
    assert_eq!(command.from, 6);
    assert_eq!(command.to, 7);
}
