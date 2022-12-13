use super::imp::{self, bfs, Matrix};

pub fn solve(input: &str) {
    let mx = Matrix::from_input(input);
    let mut min_paths: Vec<i32> = bfs(mx).into_iter().filter_map(|x| x).collect();
    min_paths.sort();
    println!("{:?} Steps", min_paths);
}
