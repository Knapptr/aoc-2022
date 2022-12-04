#[derive(Debug, Clone, Copy)]
pub struct AssignedRange {
    min: u32,
    max: u32,
}
pub fn parse_line(line: &str) -> (AssignedRange, AssignedRange) {
    let assignments = line.split(',');
    let assigned_ranges = assignments.map(|assn| {
        assn.split("-")
            .take(2)
            .map(|n| n.parse::<u32>().expect("couldnt parse number"))
            .collect::<Vec<u32>>()
    });
    let mapped_assignments: Vec<AssignedRange> = assigned_ranges
        .map(|e| AssignedRange {
            min: e[0],
            max: e[1],
        })
        .collect();
    let first_assignment = mapped_assignments[0];
    let second_assignment = mapped_assignments[1];

    (first_assignment, second_assignment)
}

pub fn test_complete_overlap(ranges: (AssignedRange, AssignedRange)) -> bool {
    let (elf1, elf2) = ranges;
    if elf1.min >= elf2.min && elf1.max <= elf2.max {
        return true;
    }
    if elf2.min >= elf1.min && elf2.max <= elf1.max {
        return true;
    }
    false
}
pub fn test_some_overlap(ranges: (AssignedRange, AssignedRange)) -> bool {
    let (elf1, elf2) = ranges;
    if elf1.min <= elf2.max && elf1.min >= elf2.min {
        return true;
    }
    if elf1.max <= elf2.max && elf1.max >= elf2.min {
        return true;
    }
    if elf2.max <= elf1.max && elf2.max >= elf1.min {
        return true;
    }
    if elf2.max <= elf1.max && elf2.max >= elf1.min {
        return true;
    }
    false
}

#[cfg(test)]
#[test]
fn parses_line_correctly() {
    let (assignment1, _assignment2) = parse_line("32-64,64-33");

    assert_eq!(assignment1.min, 32);
    assert_eq!(assignment1.max, 64);
}
