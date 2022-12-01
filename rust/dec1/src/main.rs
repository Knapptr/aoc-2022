// parse data from input file;
use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    let file = File::open("../../inputs/1.1.txt").expect("Error opening file");
    let lines: Vec<String> = io::BufReader::new(file)
        .lines()
        .filter_map(|l| l.ok())
        .collect();

    let mut elves = Elves::new();
    let mut elf = Elf::init();
    for line in lines {
        if line == "" {
            // add elf to elves, init elf
            elves.list.push(elf.clone());
            elf = Elf::init()
        } else {
            // create cal list
            let num: u32 = line.parse().expect(&format!("Error with line: {}", line));
            elf.add_snack(num);
        }
    }
    elves.get_and_print_top_3();
}

#[derive(Debug)]
struct Elf {
    snacks: Vec<u32>,
}

impl Elf {
    fn init() -> Elf {
        Elf { snacks: Vec::new() }
    }

    fn add_snack(&mut self, cal_count: u32) {
        self.snacks.push(cal_count)
    }

    fn calorie_total(&self) -> u32 {
        self.snacks.iter().sum()
    }

    fn clone(&self) -> Elf {
        Elf {
            snacks: self.snacks.clone(),
        }
    }
}

struct Elves {
    list: Vec<Elf>,
}
impl Elves {
    fn new() -> Elves {
        Elves { list: Vec::new() }
    }
    fn sort_by_calories(&mut self) {
        self.list.sort_by_key(|e| e.calorie_total())
    }

    fn get_and_print_top_3(&mut self) {
        self.sort_by_calories();
        let most = self.list.pop().unwrap().calorie_total();
        let most2 = self.list.pop().unwrap().calorie_total();
        let most3 = self.list.pop().unwrap().calorie_total();
        println!(
            "Most: {}\nSecond Most: {}\nThird Most: {}",
            most, most2, most3
        )
    }
}

// Part 2

// sort_elves(&mut elves);
// let most = elves.pop().unwrap().total_calories;
// let second_most = elves.pop().unwrap().total_calories;
// let third_most = elves.pop().unwrap().total_calories;
// println!(
//     "1st: {}\n2nd: {}\n3rd: {}\nTotal:{}",
//     most,
//     second_most,
//     third_most,
//     (most + second_most + third_most)
// );
// Part 1
// println!(
//     "Most Calories carried by single elf: {}",
//     get_max_cal_total(&elves)
// );

// fn get_max_cal_total(elves: &Vec<Elf>) -> u32 {
//     let mut most_cals = 0u32;
//     for elf in elves {
//         if elf.total_calories > most_cals {
//             most_cals = elf.total_calories;
//         }
//     }
//     most_cals
// }
