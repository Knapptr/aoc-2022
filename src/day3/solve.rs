use super::imp::Sack;

pub fn solve(input: &str) {
    // Part 1
    let mut sum = 0;
    for string in input.lines() {
        let sack = Sack::from_string(string);
        let (left_compartment, right_compartment) = sack.get_compartments_as_sacks();
        let shared_item = left_compartment.get_first_common_item_with(&right_compartment);
        sum += shared_item.get_priority();
    }
    println!("Part 1: {}", sum);
    // Part 2
    let mut sum = 0;
    let mut lines = input.lines();
    let count = lines.clone().count();
    let mut index = 0;
    while index < count {
        index += 3;
        let sack1 = Sack::from_string(lines.next().unwrap());
        let sack2 = Sack::from_string(lines.next().unwrap());
        let sack3 = Sack::from_string(lines.next().unwrap());

        let badge = sack1.get_badge_from(&sack2, &sack3);
        sum += badge.get_priority();
    }
    println!("Part 2: {}", sum)
}
