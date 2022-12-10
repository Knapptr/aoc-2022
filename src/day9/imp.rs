use std::collections::HashSet;
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

pub struct Move {
    direction: Direction,
    amount: u32,
}

impl Move {
    pub fn from_string(line: &str) -> Move {
        let (dir_str, amt_str) = line.split_once(" ").unwrap();
        let direction: Direction = match dir_str {
            "L" => Direction::Left,
            "R" => Direction::Right,
            "U" => Direction::Up,
            "D" => Direction::Down,
            _ => panic!(),
        };
        let amount: u32 = amt_str.trim().parse().expect("Not able to parse number");
        Move { direction, amount }
    }
}

type Coords = (i32, i32);
pub struct RopePlane {
    h_loc: Coords,
    knot_locs: Vec<Coords>,
    pub t_hist: Vec<Coords>,
}
impl RopePlane {
    pub fn uniq(&mut self) -> usize {
        self.t_hist.dedup();
        self.t_hist.sort();
        self.t_hist.dedup();
        self.t_hist.iter().count()
    }
    pub fn init(tail_count: usize) -> RopePlane {
        RopePlane {
            h_loc: (0, 0),
            t_hist: Vec::new(),
            knot_locs: vec![(0, 0); tail_count],
        }
    }
    pub fn update_follow_head(&mut self) {
        let first_tail = self.knot_locs.get(0).expect("there is a first tail knot");

        let new_coords = t_fol(&self.h_loc, first_tail);

        *self.knot_locs.get_mut(0).expect("there is a first value") = new_coords;
    }

    pub fn update_tail(&mut self) {
        let mut prev = self.knot_locs.first().unwrap().clone();
        for knot in self.knot_locs.iter_mut().skip(1) {
            *knot = t_fol(&prev, &knot);
            prev = *knot;
        }
    }

    fn update_history(&mut self) {
        let tail_val = self.knot_locs.last().unwrap().clone();
        self.t_hist.push(tail_val);
    }

    pub fn mv_h(&mut self, mv: Move) {
        let (x_mod, y_mod) = match mv.direction {
            Direction::Up => (0, 1),
            Direction::Down => (0, -1),
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
            _ => panic!(),
        };
        for _ in 0..mv.amount {
            self.h_loc.0 += x_mod;
            self.h_loc.1 += y_mod;
            self.update_follow_head();
            self.update_tail();
            self.update_history();
        }
    }
}
fn t_fol(last_knot_loc: &Coords, tail_knot_loc: &Coords) -> Coords {
    let difference_x = last_knot_loc.0 - tail_knot_loc.0;
    let difference_y = last_knot_loc.1 - tail_knot_loc.1;

    if difference_x.abs() <= 1 && difference_y.abs() <= 1 {
        return tail_knot_loc.clone();
    }

    let new_x = tail_knot_loc.0 + difference_x.signum();
    let new_y = tail_knot_loc.1 + difference_y.signum();

    (new_x, new_y)
}

#[cfg(test)]
#[test]
fn parses_move() {
    let test_line = "L 4 ";
    let p_move = Move::from_string(test_line);
    assert_eq!(p_move.amount, 4);
    assert!(matches!(p_move.direction, Direction::Left));
}

/// Tests
#[test]
fn moves_h() {
    let mv = Move {
        direction: Direction::Down,
        amount: 10,
    };

    let mut plane = RopePlane::init(1);

    plane.mv_h(mv);

    assert_eq!(plane.h_loc, (0, -10));
}
#[test]
fn fol_t() {
    let mv = Move {
        direction: Direction::Left,
        amount: 9,
    };
    let mut plane = RopePlane::init(1);

    plane.mv_h(mv);

    assert_eq!(plane.knot_locs[0], (-8, 0));
}
#[test]
fn fol_t_diag() {
    let mv1 = Move {
        direction: Direction::Right,
        amount: 1,
    };
    let mv2 = Move {
        direction: Direction::Up,
        amount: 2,
    };

    let mut plane = RopePlane::init(1);

    plane.mv_h(mv1);
    plane.mv_h(mv2);

    assert_eq!(plane.knot_locs[0], (1, 1));
}
#[test]
fn fol_mult() {
    let mv1 = Move {
        direction: Direction::Right,
        amount: 5,
    };

    let mut plane = RopePlane::init(2);

    plane.mv_h(mv1);

    assert_eq!(plane.knot_locs[1], (3, 0));
}

#[test]
fn count() {
    let test_in = "R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20";
    let mut plane = RopePlane::init(9);
    for line in test_in.lines() {
        let mv = Move::from_string(line);
        plane.mv_h(mv);
    }
    assert_eq!(plane.uniq(), 36);
}
