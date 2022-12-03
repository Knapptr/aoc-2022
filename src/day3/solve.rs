use super::imp::{get_badge_priority, get_sack_priority};

pub fn solve(input: &str) {
    // Part 1
    let mut sum = 0;
    for sack in input.lines() {
        sum += get_sack_priority(sack);
    }
    println!("Part 1: {}", sum);
    // Part 2
    let mut sum = 0;
    let mut lines = input.lines();
    let count = lines.clone().count();
    let mut index = 0;
    while index < count {
        index += 3;
        let sack1 = lines.next().unwrap();
        let sack2 = lines.next().unwrap();
        let sack3 = lines.next().unwrap();
        sum += get_badge_priority(sack1, sack2, sack3);
    }
    println!("Part 2: {}", sum)
}
