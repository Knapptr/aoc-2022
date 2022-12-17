use super::imp::{self, RockGrid};

const TEST_INPUT: &str = "\
498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9";

pub fn solve(input: &str) {
    // let mut cave_1 = RockGrid::from_input(input, true, None);
    let mut cave_2 = RockGrid::from_input(input, false, Some(500));
    let mut cave_1 = RockGrid::from_input(TEST_INPUT, true, None);
    // let mut cave_2 = RockGrid::from_input(TEST_INPUT, false, Some(100));
    let mut count_1 = 0;
    while cave_1.drop_possible {
        // for _x in 0..44 {
        cave_1.drop_sand_grain();
        // println!("{}", cave);
        count_1 += 1
    }
    let mut count_2 = 0;
    while cave_2.drop_possible {
        // for _x in 0..44 {
        cave_2.drop_sand_grain();
        // println!("{}", cave);
        count_2 += 1
    }

    println!("Part 1: {}", count_1 + 1);
    println!("Part 2: {}", count_2);
    // println!("{}", cave_1);
    // println!("{}", cave_2);
}
