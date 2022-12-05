use std::{collections::HashMap, fmt::Error, str::FromStr};
#[derive(Debug)]
pub struct Command {
    quantity: usize,
    from: usize,
    to: usize,
}
impl FromStr for Command {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let nums: Vec<usize> = s
            .split_whitespace()
            .filter_map(|string| string.parse().ok())
            .collect();
        Ok(Command {
            quantity: nums[0],
            from: nums[1],
            to: nums[2],
        })
    }
}

pub fn move_crates_9001(crates: &mut HashMap<usize, Vec<char>>, command: Command) {
    // splice out from len - 1 - quantity
    let mut from_stack = crates.get_mut(&command.from).unwrap();
    let start_index = from_stack.len() - command.quantity;
    let mut removed_stack: Vec<char> = from_stack.drain(start_index..).collect();
    let mut to_stack = crates.get_mut(&command.to).unwrap();
    to_stack.append(&mut removed_stack);
}
pub fn move_crates_9000(crates: &mut HashMap<usize, Vec<char>>, command: Command) {
    for _crate_num in 0..command.quantity {
        let from_stack = crates.get_mut(&command.from).unwrap();
        let moved_crate = from_stack.pop().unwrap();
        let to_stack = crates.get_mut(&command.to).unwrap();
        to_stack.push(moved_crate)
    }
}

pub fn get_stack_info(data: &str) -> String {
    let mut stack_info = String::new();
    for line in data.lines() {
        let first_char = line.trim().chars().take(1).next().unwrap();
        if first_char.is_numeric() {
            break;
        }
        stack_info.push_str(line);
        stack_info.push_str("\n");
    }
    println!("{}", stack_info);
    stack_info
}
// Must be given [Z][A][X] info only
pub fn parse_stack_input(stack_input: &str) -> Option<HashMap<usize, Vec<char>>> {
    let mut stack_map = HashMap::new();
    for line in stack_input.lines() {
        let mut stack_row: Vec<char> = Vec::new();
        let line_chars = line.chars().skip(1);
        for (index, char) in line_chars.enumerate() {
            match index % 4 {
                0 => {
                    stack_row.push(char);
                }
                _ => (),
            }
        }
        for (index, char) in stack_row.iter().enumerate() {
            let stack_number = index + 1;
            let stack = stack_map.entry(stack_number).or_insert(Vec::new());
            stack.push(char.clone());
        }
    }
    for (key, value) in stack_map.iter_mut() {
        value.reverse();
        value.retain(|x| *x != ' ');
    }
    println!("{:#?}", stack_map);
    Some(stack_map)
}
#[cfg(test)]
#[test]
fn it_parses_stacks() {
    let test_input = "\
[D]    
[N] [C]    
[Z] [M] [P]
 1   2   3";
    println!("{}", test_input);
    let stacks = parse_stack_input(&get_stack_info(test_input)).unwrap();
    assert_eq!(stacks.get_key_value(&1).expect("shit").1.len(), 3);
    assert_eq!(stacks.get_key_value(&2).expect("shit").1.len(), 2);
    assert_eq!(stacks.get_key_value(&3).expect("shit").1.len(), 1);
}
#[test]
fn parses_command() {
    let command = Command::from_str("move 1 from 4 to 8").unwrap();
    assert_eq!(command.quantity, 1);
    assert_eq!(command.from, 4);
    assert_eq!(command.to, 8);
}

#[test]
fn moves_crates() {
    let test_input = "\
[D]    
[N] [C]    
[Z] [M] [P]";
    let test_command = Command {
        quantity: 1,
        from: 3,
        to: 1,
    };
    let mut test_crates = parse_stack_input(test_input).unwrap();
    move_crates_9000(&mut test_crates, test_command);
    assert_eq!(test_crates.get_key_value(&1).expect("shit").1.len(), 4);
    assert_eq!(test_crates.get_key_value(&3).expect("shit").1.len(), 0);
}
