// use crate::day7::imp::TEST_IN;

use super::imp::Drive;

const MAX_SPACE: u32 = 70_000_000;
const DESIRED_SPACE: u32 = 30_000_000;

pub fn solve(input: &str) {
    let mut lines = input.lines();
    let root = Drive::from_input(&mut lines);

    let folder_sizes = root.get_folder_sizes();
    let part_1: u32 = folder_sizes.iter().filter(|&x| *x <= 100000).sum();

    println!("Part 1: {}", part_1);

    let current_space: u32 = root.get_size();

    let unused_space = MAX_SPACE - current_space;
    let needed_space = DESIRED_SPACE - unused_space;

    let part_2 = folder_sizes
        .iter()
        .filter(|&x| *x >= needed_space)
        .min()
        .unwrap();

    println!("Part 2: {}", part_2);
}
