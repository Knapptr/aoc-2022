use std::collections::{HashSet, VecDeque};

pub fn bfs(matrix: Matrix) -> Vec<Option<i32>> {
    let mut min_paths: Vec<Option<i32>> = Vec::new();

    for start in &matrix.starts {
        let mut q: VecDeque<(Coords, i32)> = VecDeque::new();
        let mut seen_coords = HashSet::new();
        q.push_back((*start, 0));

        while q.len() > 0 {
            let (insp_coords, steps) = q.pop_front().unwrap();

            if seen_coords.contains(&insp_coords) || steps >= min_path.unwrap_or(i32::MAX) {
                continue;
            }
            seen_coords.insert(insp_coords);

            if insp_coords == matrix.end.unwrap() {
                min_paths.push(Some(steps));
                break;
            }

            let neighbors = get_neigbor_indicies(&matrix, insp_coords);

            for (n_x, n_y) in neighbors {
                if !seen_coords.contains(&(n_x, n_y))
                    && matrix.mx[n_y][n_x].get_value()
                        - matrix.mx[insp_coords.1][insp_coords.0].get_value()
                        <= 1
                {
                    q.push_back(((n_x, n_y), steps + 1));
                }
            }
        }

        min_paths.push(None);
    }
    min_paths
}
pub type Coords = (usize, usize);

fn get_neigbor_indicies(matrix: &Matrix, coords: Coords) -> Vec<Coords> {
    let (x_index, y_index) = coords;
    let mut neighbors = Vec::new();

    let w_max = matrix.mx.get(0).expect("there is no first row").len() - 1;
    let h_max = matrix.mx.len() - 1;

    let y_range_min = if y_index == 0 { 0 } else { y_index - 1 };
    let y_range_max = if y_index == h_max { h_max } else { y_index + 1 };
    let x_range_min = if x_index == 0 { 0 } else { x_index - 1 };
    let x_range_max = if x_index == w_max { w_max } else { x_index + 1 };

    for x in x_range_min..=x_range_max {
        if x != x_index {
            neighbors.push((x, y_index));
        }
    }
    for y in y_range_min..=y_range_max {
        if y != y_index {
            neighbors.push((x_index, y));
        }
    }
    neighbors
}
#[derive(Debug)]
enum CellType {
    Start(Coords),
    End(Coords),
    Normal(i32),
}
impl CellType {
    fn get_value(&self) -> i32 {
        match self {
            CellType::Start(_) => 1,
            CellType::End(_) => 26,
            CellType::Normal(v) => *v,
        }
    }
}
pub struct Matrix {
    mx: Vec<Vec<CellType>>,
    starts: Vec<Coords>,
    end: Option<Coords>,
}

impl Matrix {
    pub fn from_input(input: &str) -> Matrix {
        let mut starts: Vec<Coords> = Vec::new();
        let mut end: Option<Coords> = None;

        let mut cell_callback = |cell: &CellType| match cell {
            CellType::End(coords) => end = Some(*coords),
            CellType::Start(coords) => starts.push(*coords),
            _ => panic!(),
        };

        let mut mx = Vec::new();

        for (y_idx, line) in input.lines().enumerate() {
            let row = parse_row(line, y_idx, &mut cell_callback);
            mx.push(row);
        }

        Matrix { mx, starts, end }
    }
}

fn parse_row<T: FnMut(&CellType)>(row: &str, col_idx: usize, mut callback: T) -> Vec<CellType> {
    let mut parsed_row = Vec::new();
    for (idx, char) in row.chars().enumerate() {
        let cell = get_letter_value(((idx, col_idx), char));
        match cell {
            CellType::End(coords) => callback(&cell),
            CellType::Start(coords) => callback(&cell),
            CellType::Normal(_) => (),
        }
        parsed_row.push(cell)
    }
    parsed_row
}

fn get_letter_value(char: (Coords, char)) -> CellType {
    match char.1 {
        'S' => CellType::Start(char.0),
        // 'a' => CellType::Start(char.0),
        'E' => CellType::End(char.0),
        _ => CellType::Normal(char.1 as i32 - 96),
    }
}
#[cfg(test)]
// #[test]
// fn gets_letter_val() {
//     assert_eq!(26, get_letter_value('E'));
// }
// #[test]
// fn parses_line_to_heights() {
//     let parsed_row = parse_row("Sabqponm");
//     assert_eq!(parsed_row, vec![1, 1, 2, 17, 16, 15, 14, 13])
// }
// #[test]
// fn gets_neighbor_coords() {
//     let m = Matrix {
//         mx: vec![vec![1, 2, 3], vec![1, 2, 3], vec![1, 2, 3]],
//     };
//     let nbs = get_neigbor_indicies(&m, (1, 1));

//     assert_eq!(nbs.len(), 4);
// }
#[test]
fn runs() {
    let m = Matrix::from_input(TEST_INPUT);

    panic!();
}

const TEST_INPUT: &str = "\
Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi";
