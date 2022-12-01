// parse data from input file;
use std::fs::File;
use std::io::{self, BufRead};

#[derive(Debug)]
struct Elf {
    calories_carried: Vec<u32>,
    total_calories: u32,
}

impl Elf {
    fn create(calorie_list: &Vec<u32>) -> Elf {
        Elf {
            total_calories: add_calories(calorie_list),
            calories_carried: calorie_list.clone(),
        }
    }
}

fn main() {
    let file = File::open("../../inputs/1.1.txt").expect("Error opening file");

    let lines: Vec<String> = io::BufReader::new(file)
        .lines()
        .filter_map(|l| l.ok())
        .collect();

    let mut calorie_list: Vec<u32> = Vec::new();
    let mut elves: Vec<Elf> = Vec::new();
    for line in lines {
        if line == "" {
            //create new elf
            let new_elf = Elf::create(&calorie_list);
            elves.push(new_elf);
            calorie_list.clear();
        } else {
            // create cal list
            let num: u32 = line.parse().expect(&format!("Error with line: {}", line));
            calorie_list.push(num);
        }
    }
    // Part 2
    sort_elves(&mut elves);
    let most = elves.pop().unwrap().total_calories;
    let second_most = elves.pop().unwrap().total_calories;
    let third_most = elves.pop().unwrap().total_calories;
    println!(
        "1st: {}\n2nd: {}\n3rd: {}\nTotal:{}",
        most,
        second_most,
        third_most,
        (most + second_most + third_most)
    );
    // Part 1
    // println!(
    //     "Most Calories carried by single elf: {}",
    //     get_max_cal_total(&elves)
    // );
}

fn add_calories(calorie_list: &Vec<u32>) -> u32 {
    let mut sum = 0u32;
    for cal_count in calorie_list {
        sum += cal_count
    }
    sum
}

fn get_max_cal_total(elves: &Vec<Elf>) -> u32 {
    let mut most_cals = 0u32;
    for elf in elves {
        if elf.total_calories > most_cals {
            most_cals = elf.total_calories;
        }
    }
    most_cals
}

fn sort_elves(elves: &mut Vec<Elf>) {
    elves.sort_by_key(|e| e.total_calories)
}
