use super::imp::parse_line_to_score;

pub fn solve(input: &str) {
    let lines = input.lines();

    let mut my_score: u32 = 0;
    for line in lines {
        my_score += parse_line_to_score(&line)
    }
    println!("{}", my_score);
}
