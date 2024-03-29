use std::env;
use std::fs;

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;
mod day20;
mod day21;
mod day22;
mod day23;
mod day24;
mod day25;

use crate::day01::DayOne;
use crate::day02::DayTwo;
use crate::day03::DayThree;
use crate::day04::DayFour;
use crate::day05::{solve_5a, solve_5b};
use crate::day06::{solve_6a, solve_6b};

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
                "7a" => day07::solve_7a("data/7.txt").to_string(),
                "7b" => day07::solve_7b("data/7.txt").to_string(),
                "8a" => day08::solve_8a("data/8.txt").to_string(),
                "8b" => day08::solve_8b("data/8.txt").to_string(),
                "9a" => day09::solve_9a("data/9.txt", 25).to_string(),
                "9b" => day09::solve_9b("data/9.txt", 25).to_string(),
                "10a" => day10::solve_10a("data/10.txt").to_string(),
                "10b" => day10::solve_10b("data/10.txt").to_string(),
                "11a" => day11::part_one("data/11.txt").to_string(),
                "11b" => day11::part_two("data/11.txt").to_string(),
                "12a" => day12::part_one("data/12.txt").to_string(),
                "12b" => day12::part_two("data/12.txt").to_string(),
                "13a" => day13::part_one("data/13.txt").to_string(),
                "13b" => day13::part_two("data/13.txt").to_string(),
                "14a" => day14::part_one("data/14.txt").to_string(),
                "14b" => day14::part_two("data/14.txt").to_string(),
                "15a" => day15::part_one(&[16,1,0,18,12,14,19], 2020).to_string(),
                "15b" => day15::part_one(&[16,1,0,18,12,14,19], 30_000_000).to_string(),
                "16a" => day16::part_one("data/16.txt").to_string(),
                "16b" => day16::part_two("data/16.txt", "departure").to_string(),
                "17a" => day17::part_one("data/17.txt").to_string(),
                "17b" => day17::part_two("data/17.txt").to_string(),
                "18a" => day18::part_one("data/18.txt").to_string(),
                "18b" => day18::part_two("data/18.txt").to_string(),
                "19a" => day19::part_one("data/19rules.txt", "data/19inputs.txt").to_string(),
                "19b" => day19::part_two("data/19rules.txt", "data/19inputs.txt").to_string(),
                "20a" => day20::part_one("data/20.txt").to_string(),
                "20b" => day20::part_two("data/20.txt").to_string(),
                "21a" => day21::part_one("data/21.txt").to_string(),
                "21b" => day21::part_two("data/21.txt").to_string(),
                "22a" => day22::part_one("data/22_1.txt", "data/22_2.txt").to_string(),
                "22b" => day22::part_two("data/22_1.txt", "data/22_2.txt").to_string(),
                "23a" => day23::part_one("685974213", 100).to_string(),
                "23b" => day23::part_two("685974213", 10_000_000, 1_000_000).to_string(),
                "24a" => day24::part_one("data/24.txt").to_string(),
                "24b" => day24::part_two("data/24.txt", 100).to_string(),
                "25a" => day25::part_one(10604480, 4126658).to_string(),
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
