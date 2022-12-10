const test_input: &str = "\
30373
25512
65332
33549
35390";

pub fn find_highest_score(grid: &TreeGrid) -> u32 {
    let mut score: u32 = 0;
    for (y_idx, row) in grid.iter().enumerate() {
        for (x_idx, _) in row.iter().enumerate() {
            let scenic_score = get_full_score(&grid, y_idx, x_idx);
            if scenic_score > score {
                score = scenic_score;
            }
        }
    }
    score
}
pub fn get_full_score(grid: &TreeGrid, y_index: usize, x_index: usize) -> u32 {
    let top_score = get_top_score(grid, y_index, x_index);
    let bottom_score = get_bottom_score(grid, y_index, x_index);
    let row = grid.get(y_index).unwrap();
    let left_score = get_left_score(row, x_index);
    let right_score = get_right_score(row, x_index);
    top_score * bottom_score * left_score * right_score
}
fn get_bottom_score(grid: &TreeGrid, mut y_index: usize, x_index: usize) -> u32 {
    if y_index == grid.len() - 1 {
        return 0;
    }
    let mut score: u32 = 0;
    let tree_height = grid.get(y_index).unwrap().get(x_index).unwrap();
    y_index += 1;
    loop {
        let current_row = grid.get(y_index).unwrap();
        let current_el = current_row.get(x_index).unwrap();
        score += 1;
        if current_el >= tree_height {
            break;
        }
        if y_index == grid.len() - 1 {
            break;
        }
        y_index += 1;
    }
    score
}
fn get_top_score(grid: &TreeGrid, mut y_index: usize, x_index: usize) -> u32 {
    if y_index == 0 {
        return 0;
    }
    let mut score: u32 = 0;
    let tree_height = grid.get(y_index).unwrap().get(x_index).unwrap();
    y_index -= 1;
    loop {
        let current_row = grid.get(y_index).unwrap();
        let current_el = current_row.get(x_index).unwrap();
        score += 1;
        if current_el >= tree_height {
            break;
        }
        if y_index == 0 {
            break;
        }
        y_index -= 1;
    }
    score
}
fn get_left_score(row: &TreeRow, mut index: usize) -> u32 {
    let mut score: u32 = 0;
    if index == 0 {
        return 0;
    }
    let tree_height = row.get(index).unwrap();
    let mut compare_index = index - 1;
    let mut current_el: u32 = *row.get(compare_index).unwrap();
    loop {
        score += 1;
        if current_el >= *tree_height {
            break;
        }
        if compare_index == 0 {
            break;
        }
        compare_index -= 1;
        current_el = *row.get(compare_index).unwrap();
    }
    score
}
fn get_right_score(row: &TreeRow, mut index: usize) -> u32 {
    let mut score: u32 = 0;
    if index == row.len() - 1 {
        return 0;
    }
    let tree_height = row.get(index).unwrap();
    let mut compare_index = index + 1;
    let mut current_el: u32 = *row.get(compare_index).unwrap();
    loop {
        score += 1;
        if current_el >= *tree_height {
            break;
        }
        if compare_index == row.len() - 1 {
            break;
        }
        compare_index += 1;
        current_el = *row.get(compare_index).unwrap();
    }
    score
}
fn test_horizontal(row: &TreeRow, index: usize) -> bool {
    // if index == 0 || index == row.len() - 1 {
    //     return true;
    // }
    let test_height = row.get(index).unwrap();
    let (before_trees, after_trees) = row.split_at(index);
    //test before
    if before_trees.iter().any(|b| b >= test_height)
        && after_trees.iter().skip(1).any(|a| a >= test_height)
    {
        return false;
    }
    true
}

fn test_vertical(grid: &TreeGrid, x_index: usize, y_index: usize) -> bool {
    // if y_index == 0 || y_index == grid.len() - 1 {
    //     return true;
    // }
    let test_height = grid.get(y_index).unwrap().get(x_index).unwrap();
    let (grid_before, grid_after) = grid.split_at(y_index);
    if grid_before
        .iter()
        .any(|x| x.get(x_index).unwrap() >= test_height)
        && grid_after
            .iter()
            .skip(1)
            .any(|x| x.get(x_index).unwrap() >= test_height)
    {
        return false;
    }
    true
}

fn parse_line(line_input: &str) -> Vec<u32> {
    line_input
        .split("")
        .filter_map(|x| x.parse().ok())
        .collect()
}

pub fn parse_grid_from_input(input: &str) -> Vec<Vec<u32>> {
    let mut grid: Vec<Vec<u32>> = Vec::new();
    for line in input.lines() {
        grid.push(parse_line(line));
    }
    grid
}

pub fn count_visible_trees(grid: &TreeGrid) -> usize {
    let mut amt = (grid.len() * 2) + (grid.get(0).unwrap().len() * 2) - 4;

    // start at all outside vals
    for (y_index, row) in grid.iter().enumerate().skip(1).take(grid.len() - 2) {
        for (x_index, x_val) in row.iter().enumerate().skip(1).take(row.len() - 2) {
            if test_horizontal(row, x_index) {
                amt += 1
            } else {
                if test_vertical(&grid, x_index, y_index) {
                    amt += 1
                }
            }
        }
    }
    amt
}

type TreeRow = Vec<u32>;
type TreeGrid = Vec<TreeRow>;
#[cfg(test)]
#[test]
fn line_to_vec() {
    let parsed_vec = parse_line("30373");
    assert_eq![parsed_vec, vec![3, 0, 3, 7, 3]]
}

#[test]
fn parse_grid() {
    let parsed_grid = parse_grid_from_input("30327\n25512");
    assert_eq!(parsed_grid, vec![vec![3, 0, 3, 2, 7], vec![2, 5, 5, 1, 2]])
}
#[test]
fn check_horizontal() {
    let row = parse_line("65332");
    let is_horizontal_visible = test_horizontal(&row, 1);
    assert!(is_horizontal_visible);
}
#[test]
fn check_vertical() {
    let grid = parse_grid_from_input(test_input);
    let is_vertical_visible = test_vertical(&grid, 1, 1);
    assert!(is_vertical_visible);
}
#[test]
fn counts_trees() {
    let grid = parse_grid_from_input(test_input);
    let count = count_visible_trees(&grid);
    assert_eq!(count, 21);
}

#[test]
fn gets_left_score() {
    let row = parse_line("184");
    let left_score = get_left_score(&row, 2);
    assert_eq!(left_score, 1)
}
#[test]
fn gets_right_score() {
    let row = parse_line("184559");
    let right_score = get_right_score(&row, 1);
    assert_eq!(right_score, 4)
}
#[test]
fn gets_top_score() {
    let grid = parse_grid_from_input(test_input);

    let score = get_top_score(&grid, 3, 2);

    assert_eq!(score, 2)
}
#[test]
fn gets_bottom_score() {
    let grid = parse_grid_from_input(test_input);

    let score = get_bottom_score(&grid, 1, 2);

    assert_eq!(score, 2)
}
#[test]
fn gets_full_score() {
    let grid = parse_grid_from_input(test_input);

    let score = get_full_score(&grid, 3, 2);

    assert_eq!(score, 8)
}
