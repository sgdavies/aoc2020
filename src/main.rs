use std::collections::BTreeMap;
use std::env;
use std::fs;

mod day_one;
mod day_two;
use crate::day_one::day_one::DayOne;
use crate::day_two::day_two::DayTwo;

extern crate lazy_static;
extern crate regex;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("Hello, world! {:?}", args);

    let mut days: BTreeMap<&str, Box<dyn Solve + Send + Sync>> = BTreeMap::new();
    days.insert("1", Box::new(DayOne::default()));
    days.insert("2a", Box::new(DayTwo::default()));

    let days_to_test: Vec<&str> = match args.len() {
        0 => panic!("Expected at least name of target!"),
        1 => days.keys().cloned().collect(),
        _ => args[1..].iter().map(AsRef::as_ref).collect(),
    };

    for day in days_to_test {
        match days.get(day) {
            Some(solver) => println!("Day {} - {:?}", day, solver.solve()),
            None => panic!("No target for '{}'", day),
        };
    }
}

pub trait Solve {
    fn solve(&self) -> i32;
}

/// Utils
// Read data from a file and return a vec of 1 string per line
pub fn file_to_vec(filename: &str) -> Vec<String> {
    let contents = fs::read_to_string(filename).expect(&format!("Couldn't read file {}", filename));
    contents.split("\n").map(str::to_string).collect()
}