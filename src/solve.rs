use aoc::{day1, day2};

pub fn solve(day: usize, input: &str) {
    match day {
        1 => day1::solve(input),
        2 => day2::solve(input),
        _ => panic!(),
    }
}
