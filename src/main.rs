use std::env;
use std::fs;

mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day_eight;
mod day_five;
mod day_four;
mod day_nine;
mod day_one;
mod day_seven;
mod day_six;
mod day_ten;
mod day_three;
mod day_two;
use crate::day_five::{solve_5a, solve_5b};
use crate::day_four::DayFour;
use crate::day_one::DayOne;
use crate::day_six::{solve_6a, solve_6b};
use crate::day_three::DayThree;
use crate::day_two::DayTwo;

extern crate lazy_static;
extern crate regex;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("Hello, world! {:?}", args);

    let days_to_test: Vec<&str> = match args.len() {
        0 => panic!("Expected at least name of target!"),
        // 1 => days.keys().cloned().collect(), // List of all keys
        _ => args[1..].iter().map(AsRef::as_ref).collect(),
    };

    for day in days_to_test {
        println!(
            "Day {}: -> {}",
            day,
            match day {
                "1a" => DayOne::default().solve_one().to_string(),
                "1b" => DayOne::default().solve_two().to_string(),
                "2a" => DayTwo::default().valid_one().to_string(),
                "2b" => DayTwo::default().valid_two().to_string(),
                "3a" => DayThree::default().solve_one().to_string(),
                "3b" => DayThree::default().solve_two().to_string(),
                "4a" => DayFour::default().solve_one().to_string(),
                "4b" => DayFour::default().solve_two().to_string(),
                "5a" => solve_5a().to_string(),
                "5b" => solve_5b().to_string(),
                "6a" => solve_6a("data/6.txt").to_string(),
                "6b" => solve_6b("data/6.txt").to_string(),
                "7a" => day_seven::solve_7a("data/7.txt").to_string(),
                "7b" => day_seven::solve_7b("data/7.txt").to_string(),
                "8a" => day_eight::solve_8a("data/8.txt").to_string(),
                "8b" => day_eight::solve_8b("data/8.txt").to_string(),
                "9a" => day_nine::solve_9a("data/9.txt", 25).to_string(),
                "9b" => day_nine::solve_9b("data/9.txt", 25).to_string(),
                "10a" => day_ten::solve_10a("data/10.txt").to_string(),
                "10b" => day_ten::solve_10b("data/10.txt").to_string(),
                "11a" => day11::part_one("data/11.txt").to_string(),
                "11b" => day11::part_two("data/11.txt").to_string(),
                "12a" => day12::part_one("data/12.txt").to_string(),
                "12b" => day12::part_two("data/12.txt").to_string(),
                "13a" => day13::part_one("data/13.txt").to_string(),
                "13b" => day13::part_two("data/13.txt").to_string(),
                "14a" => day14::part_one("data/14.txt").to_string(),
                "14b" => day14::part_two("data/14.txt").to_string(),
                "15a" => day15::part_one(&[6, 3, 15, 13, 1, 0], 2020).to_string(),
                "15b" => day15::part_one(&[6, 3, 15, 13, 1, 0], 30000000).to_string(),
                _ => panic!("No target for '{}'", day),
            }
        );
    }
}

/// Utils
// Read data from a file and return a vec of 1 string per line
pub fn file_to_vec(filename: &str) -> Vec<String> {
    let contents =
        fs::read_to_string(filename).unwrap_or_else(|_| panic!("Couldn't read file {}", filename));
    contents.split('\n').map(str::to_string).collect()
}
