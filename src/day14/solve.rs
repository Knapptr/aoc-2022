use super::imp::{self, RockGrid};

const TEST_INPUT: &str = "\
498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9";

pub fn solve(input: &str) {
    let mut cave = RockGrid::from_input(input);
    // let mut cave = RockGrid::from_input(TEST_INPUT);
    let mut count = 0;
    while cave.drop_possible {
        cave.drop_sand_grain();
        count += 1
    }
    // println!("{}", cave);
    println!("Part 1: {}", count - 1);
}
