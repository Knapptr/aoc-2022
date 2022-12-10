use super::imp::{Move, RopePlane};

pub fn solve(input: &str) {
    // part 1
    let mut single_plane = RopePlane::init(1);

    for line in input.lines() {
        let mv = Move::from_string(line);
        single_plane.mv_h(mv);
    }
    println!("Part 1: {}", single_plane.uniq());

    // part 2
    let mut multi_plane = RopePlane::init(9);
    for line in input.lines() {
        let mv = Move::from_string(line);
        multi_plane.mv_h(mv);
    }
    println!("Part 2: {}", multi_plane.uniq());
}
