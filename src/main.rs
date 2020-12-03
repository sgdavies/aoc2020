use std::env;
use std::fs;

mod template;
mod day_one;
mod day_two;
use crate::day_one::DayOne;
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
            "Day {}: -> {:?}",
            day,
            match day {
                "1a" => DayOne::default().solve_one(),
                "1b" => DayOne::default().solve_two(),
                "2a" => DayTwo::default().valid_one(),
                "2b" => DayTwo::default().valid_two(),
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
