use super::imp::{self, parse_line, score};

pub fn solve() {
    let lines = imp::read_file();

    let mut my_score: u32 = 0;
    for line in lines {
        my_score += score(&parse_line(&line));
    }
    println!("{}", my_score);
}
