use super::imp::{Commands, Ship};

pub fn solve(input: &str) {
    let (ship_info, commands_input) = input.split_once("\n\n").expect("Data is uniform");

    // Part 1
    // create ship
    let mut ship_9000 = Ship::from_input(ship_info);
    // create command list
    let commands = Commands::from_input(commands_input);
    // move all crates
    for command in commands.list {
        ship_9000.move_9000(command)
    }
    // Part 2
    // create ship
    let mut ship_9001 = Ship::from_input(ship_info);
    // create command list
    let commands = Commands::from_input(commands_input);
    // move all crates
    for command in commands.list {
        ship_9001.move_9001(command)
    }
    // Print Answers
    println!("part 1: {}", ship_9000.top_crates());
    println!("part 2: {}", ship_9001.top_crates());
}
