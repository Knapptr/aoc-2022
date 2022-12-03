use std::collections::HashMap;

pub fn get_sack_priority(sack_string: &str) -> u32 {
    let mut halves = split_sack(sack_string);
    let right = halves.pop().unwrap();
    let left = halves.pop().unwrap();
    let duplicate_item = get_duplicate(right, left);
    get_priority(duplicate_item)
}
pub fn get_badge_priority(sack1: &str, sack2: &str, sack3: &str) -> u32 {
    let items1 = get_items(sack1);
    let items2 = get_items(sack2);
    let items3 = get_items(sack3);
    let badge = get_badge(items1, items2, items3);
    get_priority(badge)
}
fn get_items(sack: &str) -> Vec<char> {
    sack.chars().collect::<Vec<char>>()
}
fn get_badge(sack1: Vec<char>, sack2: Vec<char>, sack3: Vec<char>) -> char {
    let duplicates_1_2 = get_duplicates(sack1, sack2);
    get_duplicate(duplicates_1_2, sack3)
}

fn get_duplicates(sack1: Vec<char>, sack2: Vec<char>) -> Vec<char> {
    let mut duplicates: Vec<char> = Vec::new();
    let mut map_1 = HashMap::new();
    for char in sack1 {
        map_1.insert(char, 1);
    }
    for char in sack2 {
        match map_1.get(&char) {
            Some(_n) => duplicates.push(char),
            None => (),
        }
    }
    duplicates
}

fn split_sack(string: &str) -> Vec<Vec<char>> {
    //get char count
    let chars: Vec<char> = string.chars().collect(); // handle utf8
    let midpoint = chars.len() / 2;
    //split at midpoint
    let left: Vec<char> = chars[0..midpoint].to_vec();
    let right: Vec<char> = chars[midpoint..].to_vec();
    //return array of halves
    Vec::from([left, right])
}
fn get_priority(char: char) -> u32 {
    if !char.is_ascii_alphabetic() {
        // handle any non latin chars
        panic!()
    }
    // just really want to figure out this syntax.
    // could be a ternary
    if let true = char.is_uppercase() {
        char as u32 - 38
    } else {
        char as u32 - 96
    }
}
fn get_duplicate(compartment1: Vec<char>, compartment2: Vec<char>) -> char {
    // map 1
    let mut char_map_1 = HashMap::new();
    for char in compartment1 {
        char_map_1.insert(char, 1);
    }
    for char in compartment2 {
        match char_map_1.get(&char) {
            Some(_c) => return char,
            None => (),
        }
    }
    panic!()
}
#[cfg(test)]
#[test]
fn splits_sack_correctly() {
    let sack = "abcdef";
    let halves = split_sack(sack);
    assert_eq!(halves[0].len(), 3);
    assert_eq!(halves[1].len(), 3);
}

#[test]
fn gets_priority_correctly() {
    let char_a = 'a';
    let char_z = 'z';
    let char_ua = 'A';
    let char_uz = 'Z';
    assert_eq!(get_priority(char_a), 1);
    assert_eq!(get_priority(char_z), 26);
    assert_eq!(get_priority(char_ua), 27);
    assert_eq!(get_priority(char_uz), 52);
}

#[test]
#[should_panic]
fn panics_on_non_alpha() {
    get_priority('?');
    get_priority('🔎');
}

#[test]
fn returns_duplicate_char() {
    let comp_1 = vec!['a', 'b', 'c', 'd'];
    let comp_2 = vec!['e', 'f', 'g', 'a'];

    assert_eq!(get_duplicate(comp_1, comp_2), 'a')
}
