use super::imp::{count_visible_trees, find_highest_score, parse_grid_from_input};

pub fn solve(input: &str) {
    let tree_grid = parse_grid_from_input(input);
    let count = count_visible_trees(&tree_grid);
    println!("Part 1: {}", count);
    let score = find_highest_score(&tree_grid);
    println!("Part 2: {}", score);
}
