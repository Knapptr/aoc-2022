use super::imp;
use std::str::FromStr;

pub fn solve(input: &str) {
    let mut input_sections = input.split("\n\n");
    let ship_section = input_sections.next().unwrap();
    let command_section = input_sections.next().unwrap();

    // parse ship
    let mut ship_crates = imp::parse_stack_input(&imp::get_stack_info(ship_section)).unwrap();

    // move crates
    for command_line in command_section.lines() {
        let command = imp::Command::from_str(command_line).unwrap();
        imp::move_crates_9001(&mut ship_crates, command);
        // imp::move_crates_9000(&mut ship_crates, command);
    }
    // print ship
    // count stacks
    let stack_count = ship_crates.iter().count();
    for stack_number in 1..=stack_count {
        //print the last item on the array
        let last_item = ship_crates.get_mut(&stack_number).unwrap().pop().unwrap();

        println!("Stack {}: {}", stack_number, last_item)
    }
}
