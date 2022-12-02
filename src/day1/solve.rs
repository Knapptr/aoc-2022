use super::imp::{Elf, Elves};
use std::fs::File;
use std::io::{self, BufRead};

pub fn solve() {
    let file = File::open("inputs/1.1.txt").expect("Error opening file");
    let lines: Vec<String> = io::BufReader::new(file)
        .lines()
        .filter_map(|l| l.ok())
        .collect();

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
