use crate::day7::imp::TEST_IN;

use super::imp::Drive;

// use super::imp::{parse_line, Command, Directory, LineType};
#[allow(unused_variables)]
pub fn solve(input: &str) {
    let root = Drive::from_input(input);
    // let root = Drive::from_input(TEST_IN);
    let sum: u32 = get_folder_sizes(root).iter().sum();
    println!("part 1: {}", sum);
}
fn get_folder_sizes(drive: Drive) -> Vec<u32> {
    let mut folder_list = Vec::new();
    if drive.get_size() <= 100_000 {
        folder_list.push(drive.get_size());
    }
    for folder in drive.folders {
        let res = get_folder_sizes(folder);
        for val in res {
            folder_list.push(val);
        }
    }
    folder_list
}
