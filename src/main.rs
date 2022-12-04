#[allow(unused_imports)]
use aoc::{day1, day2, day3, day4};
use chrono::{Datelike, Local};
use solve::solve;
use std::{env, fs};

mod solve;
fn main() {
    // Parse arguments
    let arg_input: Vec<String> = env::args().collect();
    let args = parse_args(&arg_input).unwrap_or(Args {
        day_number: 1,
        file_name: "inputs/1.txt".to_string(),
    }); // Set default with no arguments to day 1.

    println!("Solving day {}...", args.day_number);

    // Get File String
    let file_string = read_input(&args).expect("Error reading file. Did you save the inputs?");

    // Solve
    solve(args.day_number, &file_string);
}

#[derive(Debug)]
struct Args {
    day_number: usize,
    file_name: String,
}

fn parse_args(args: &Vec<String>) -> Result<Args, &str> {
    if args.len() != 2 {
        let day_number = get_todays_number() as usize;
        return Ok(Args {
            day_number,
            file_name: format!("inputs/{}.txt", day_number),
        });
    }
    let day_number: usize = match args[1].parse() {
        Ok(num) => num,
        Err(_e) => return Err("Error parsing day number from input"),
    };
    let file_name = format!("inputs/{}.txt", day_number);
    Ok(Args {
        day_number,
        file_name,
    })
}
fn read_input(args: &Args) -> Result<String, &str> {
    let file_contents = fs::read_to_string(&args.file_name);
    match file_contents {
        Ok(string) => return Ok(string),
        Err(_e) => return Err("error reading file. (Did you save the input?)"),
    }
}

fn get_todays_number() -> u32 {
    let now = Local::now();
    now.day0() + 1
}
