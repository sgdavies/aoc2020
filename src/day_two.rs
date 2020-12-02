
#[macro_use]

pub mod day_two {
    use crate::{file_to_vec, Solve};
    use lazy_static::lazy_static;
    use regex::Regex;

    lazy_static! {
        // Example: "8-10 q: qqqqqqqtqq"
        static ref RECORD: Regex = Regex::new(r"^(\d+)-(\d+) ([a-z]): ([a-z]+)$").unwrap();
    }

    struct Record {
        letter: char,
        min: usize,
        max: usize,
        password: String,
    }
    
    impl Record {
        fn from(record_str: &str) -> Record {
            let record: Record = RECORD.captures(record_str).ok_or(format!("Couldn't parse input: '{}'", record_str)).and_then(|cap| {
                    Ok(Record {
                        letter: cap[3].chars().next().unwrap(),
                        min: cap[1].parse::<usize>().unwrap(),
                        max: cap[2].parse::<usize>().unwrap(),
                        password: cap[4].to_string(),
                    }
            )}).unwrap();

            record
        }

        fn valid(&self) -> bool {
            let count = self.password.chars().filter(|c| *c==self.letter).count();
            (self.min <= count) && (count <= self.max)
        }
    }

    pub struct DayTwo {
        records: Vec<Record>,
    }

    impl DayTwo {
        fn new(filename: &str) -> Self {
            DayTwo {
                records: file_to_vec(filename).iter().map(|s| Record::from(s)).collect(),
            }
        }

        pub fn default() -> Self {
            DayTwo::new("data/2a.txt")
        }

        fn valid_records(&self) -> i32 {
            self.records.iter().filter(|r| r.valid()).count() as i32
        }
    }

    impl Solve for DayTwo {
        fn solve(&self) -> i32 {
            self.valid_records()
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test() {
            let day = DayTwo::new("data/2a_example.txt");
            assert!(day.valid_records() == 2);
        }
    }
}