use std::fs::read_to_string;

pub fn read_file() -> Vec<String> {
    let file_contents = read_to_string("inputs/2.txt").expect("could not read data in 2.txt");

    file_contents.lines().map(|x| x.to_string()).collect()
}

pub type RoundResults = (ThrownObject, ThrownObject);
pub enum ThrownObject {
    Rock,
    Paper,
    Scissors,
}
impl ThrownObject {
    fn from_char(char: char) -> ThrownObject {
        match char {
            'A' => ThrownObject::Rock,
            'B' => ThrownObject::Paper,
            'C' => ThrownObject::Scissors,
            _ => unreachable!(),
        }
    }
    fn from_suggestion(opponent_throws: char, suggested_condition: char) -> ThrownObject {
        let opponent: ThrownObject = ThrownObject::from_char(opponent_throws);
        match suggested_condition {
            'X' => match opponent {
                // need to lose
                ThrownObject::Scissors => ThrownObject::Paper,
                ThrownObject::Paper => ThrownObject::Rock,
                ThrownObject::Rock => ThrownObject::Scissors,
            },
            'Z' => match opponent {
                // need to win
                ThrownObject::Scissors => ThrownObject::Rock,
                ThrownObject::Paper => ThrownObject::Scissors,
                ThrownObject::Rock => ThrownObject::Paper,
            },
            _ => opponent, // will not handle code that doesn't conform to spec
        }
    }
}

pub fn parse_line(line: &str) -> RoundResults {
    let mut letters = line.split(" ").map(|x| x.chars());
    let el1 = letters
        .next()
        .expect("something went wrong")
        .nth(0)
        .unwrap();
    let el2 = letters
        .next()
        .expect("something went wrong")
        .nth(0)
        .unwrap();
    (
        ThrownObject::from_char(el1),
        ThrownObject::from_suggestion(el1, el2),
    )
}
#[derive(Debug)]
enum Result {
    Win,
    Lose,
    Tie,
}

fn do_i_win(throws: &RoundResults) -> Result {
    match throws.1 {
        ThrownObject::Rock => match throws.0 {
            ThrownObject::Rock => Result::Tie,
            ThrownObject::Paper => Result::Lose,
            ThrownObject::Scissors => Result::Win,
        },
        ThrownObject::Paper => match throws.0 {
            ThrownObject::Rock => Result::Win,
            ThrownObject::Paper => Result::Tie,
            ThrownObject::Scissors => Result::Lose,
        },
        ThrownObject::Scissors => match throws.0 {
            ThrownObject::Rock => Result::Lose,
            ThrownObject::Paper => Result::Win,
            ThrownObject::Scissors => Result::Tie,
        },
    }
}

pub fn score(throws: &RoundResults) -> u32 {
    let i_win: Result = do_i_win(&throws);
    let thrown_points: u32 = match throws.1 {
        ThrownObject::Rock => 1,
        ThrownObject::Paper => 2,
        ThrownObject::Scissors => 3,
    };
    let result_points: u32 = match i_win {
        Result::Win => 6,
        Result::Tie => 3,
        Result::Lose => 0,
    };
    thrown_points + result_points
}

#[cfg(test)]
#[test]
fn parse_line_returns_correctly() {
    let str = "A X";
    let result = parse_line(str);
    assert!(matches!(result.1, ThrownObject::Scissors));
    assert!(matches!(result.0, ThrownObject::Rock))
}

#[test]
fn scores_correctly() {
    let mut throws: RoundResults = (ThrownObject::Rock, ThrownObject::Scissors);
    assert_eq!(score(&throws), 3);
    throws.0 = ThrownObject::Scissors;
    assert_eq!(score(&throws), 6);
}
