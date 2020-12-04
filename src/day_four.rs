use std::collections::HashMap;
use std::fs;

pub struct DayFour {
    filename: String,
}

impl DayFour {
    fn new(filename: &str) -> Self {
        DayFour {
            filename: filename.to_string(),
        }
    }

    pub fn default() -> Self {
        DayFour::new("data/4a.txt")
    }

    pub fn solve_one(&self) -> i32 {
        let contents = fs::read_to_string(&self.filename)
            .unwrap_or_else(|_| panic!("Couldn't read file {}", self.filename));
        let v: Vec<Vec<String>> = contents
            .split("\n\n")
            .map(|s| {
                s.to_string()
                    .split(|c| c == ' ' || c == '\n')
                    .map(str::to_string)
                    .collect()
            })
            .collect();

        let v: Vec<HashMap<&str, &str>> = v
            .iter()
            .map(|inner_vec| {
                let mut map: HashMap<&str, &str> = HashMap::new();
                for s in inner_vec.iter() {
                    let parts: Vec<&str> = s.split(':').collect();
                    map.insert(parts[0], parts[1]);
                }
                map
            })
            .collect();

        let required_keys = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
        v.iter().fold(0, |acc, map| {
            acc + if required_keys.iter().all(|&x| map.contains_key(x)) {
                1
            } else {
                0
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let four = DayFour::new("data/4a_example.txt");
        assert!(four.solve_one() == 2);
    }
}
