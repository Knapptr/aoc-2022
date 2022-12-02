use std::fs::read_to_string;

pub fn read_file() -> Vec<String> {
    let file_contents = read_to_string("inputs/2.txt").expect("could not read data in 2.txt");

    file_contents.lines().map(|x| x.to_string()).collect()
}

#[derive(Debug)]
enum Result {
    Win,
    Lose,
    Tie,
}
pub enum ThrownObject {
    Rock,
    Paper,
    Scissors,
}
impl ThrownObject {
    // use this to solve part 1 when parsing
    // XYZ as Rock, paper, Scissors
    #[allow(dead_code)]
    fn from_char_p1(char: char) -> ThrownObject {
        match char {
            'X' => ThrownObject::Rock,
            'Y' => ThrownObject::Paper,
            'Z' => ThrownObject::Scissors,
            _ => unreachable!(),
        }
    }
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
    fn do_i_win_against(&self, against: &ThrownObject) -> Result {
        match self {
            ThrownObject::Rock => match against {
                ThrownObject::Rock => Result::Tie,
                ThrownObject::Paper => Result::Lose,
                ThrownObject::Scissors => Result::Win,
            },
            ThrownObject::Paper => match against {
                ThrownObject::Rock => Result::Win,
                ThrownObject::Paper => Result::Tie,
                ThrownObject::Scissors => Result::Lose,
            },
            ThrownObject::Scissors => match against {
                ThrownObject::Rock => Result::Lose,
                ThrownObject::Paper => Result::Win,
                ThrownObject::Scissors => Result::Tie,
            },
        }
    }
    fn score_against(&self, against: &ThrownObject) -> u32 {
        let i_win: Result = self.do_i_win_against(&against);
        let thrown_points: u32 = match self {
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
}

pub fn parse_line_to_score(line: &str) -> u32 {
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

    let their_move = ThrownObject::from_char(el1);
    let my_move = ThrownObject::from_suggestion(el1, el2);

    my_move.score_against(&their_move)
}
