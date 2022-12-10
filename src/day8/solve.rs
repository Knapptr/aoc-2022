use crate::day8::imp::{find_highest_score, get_full_score};

use super::imp::{count_visible_trees, parse_grid_from_input};
pub fn solve(input: &str) {
    let tree_grid = parse_grid_from_input(input);
    let count = count_visible_trees(&tree_grid);
    println!("Part 1: {}", count);
    let score = find_highest_score(&tree_grid);
    println!("Part 2: {}", score);
}
