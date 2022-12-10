use aoc::{day1, day2, day3, day4, day5, day6, day7, day8, day9};

pub fn solve(day: usize, input: &str) {
    match day {
        1 => day1::solve(input),
        2 => day2::solve(input),
        3 => day3::solve(input),
        4 => day4::solve(input),
        5 => day5::solve(input),
        6 => day6::solve(input),
        7 => day7::solve(input),
        8 => day8::solve(input),
        9 => day9::solve(input),
        _ => panic!(),
    }
}
