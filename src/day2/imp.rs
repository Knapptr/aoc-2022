use std::fs::read_to_string;

pub fn read_file() -> Vec<String> {
    let file_contents = read_to_string("inputs/2.txt").expect("could not read data in 2.txt");

    file_contents.lines().map(|x| x.to_string()).collect()
}

pub enum ThrownObject {
    Rock,
    Paper,
    Scissors,
}
pub type RoundResults = (ThrownObject, ThrownObject);

fn get_thrown(char_thrown: char) -> ThrownObject {
    match char_thrown {
        'X' | 'A' => ThrownObject::Rock,
        'Y' | 'B' => ThrownObject::Paper,
        'Z' | 'C' => ThrownObject::Scissors,
        _ => panic!(),
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
    let opponent_throw: ThrownObject = get_thrown(el1);
    let opponent_throw_for_other_function: ThrownObject = get_thrown(el1);
    (
        opponent_throw,
        get_thrown_p2(opponent_throw_for_other_function, el2),
    )
}

fn get_thrown_p2(opponent: ThrownObject, suggested_condition: char) -> ThrownObject {
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
        _ => opponent, // need to tie. This will not handle data that does not corform to
                       // the xyz spec
    }
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
fn get_thrown_returns_correctly() {
    let mut thrown_char = 'A';
    assert!(matches!(get_thrown(thrown_char), ThrownObject::Rock));
    thrown_char = 'B';
    assert!(matches!(get_thrown(thrown_char), ThrownObject::Paper));
    thrown_char = 'C';
    assert!(matches!(get_thrown(thrown_char), ThrownObject::Scissors));
}

#[test]
#[should_panic]
fn get_thrown_panics_on_non_char() {
    get_thrown('T');
}

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
