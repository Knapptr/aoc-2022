use super::imp::{parse_line, test_complete_overlap, test_some_overlap};

pub fn solve(input: &str) {
    let lines = input.lines();

    let mut complete_sum: u32 = 0;
    let mut any_sum: u32 = 0;

    for line in lines {
        let pair_assignments = parse_line(line);
        if test_complete_overlap(pair_assignments) {
            complete_sum += 1;
        }
        if test_some_overlap(pair_assignments) {
            any_sum += 1;
        }
    }
    println!("Part 1: {} pairs completely overlap.", complete_sum);
    println!("Part 2: {} pairs partially overlap.", any_sum);
}
