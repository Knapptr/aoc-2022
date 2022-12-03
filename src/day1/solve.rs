use super::imp::{Elf, Elves};

pub fn solve(input: &str) {
    let lines = input.lines();

    let mut elves = Elves::new();
    let mut elf = Elf::init();
    for line in lines {
        if line == "" {
            // add elf to elves, init elf
            elves.add_elf(&elf);
            elf = Elf::init()
        } else {
            // create cal list
            let num: u32 = line.parse().expect(&format!("Error with line: {}", line));
            elf.add_snack(num);
        }
    }
    elves.get_and_print_top_3();
}
