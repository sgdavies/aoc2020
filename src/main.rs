use std::env;
use std::fs;

mod day_one;
use crate::day_one::day_one::DayOne;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("Hello, world! {:?}", args);

    DayOne::solve(&args);
}

pub trait Solve {
    fn solve(args: &Vec<String>);
}

pub trait Test {
    fn test(&self, args: &Vec<String>);
}

// Read data from a file and return a vec of 1 string per line
pub fn file_to_vec(filename: &str) -> Vec<String> {
    let contents = fs::read_to_string(filename).expect(&format!("Couldn't read file {}", filename));
    contents.split("\n").map(str::to_string).collect()
}
