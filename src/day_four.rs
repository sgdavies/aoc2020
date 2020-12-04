use std::collections::HashMap;
use std::fs;

pub struct DayFour {
    passports: Vec<Passport>,
}

impl DayFour {
    fn new(filename: &str) -> Self {
        let contents = fs::read_to_string(filename)
            .unwrap_or_else(|_| panic!("Couldn't read file {}", filename));
        let passports: Vec<Passport> = contents.split("\n\n").map(Passport::new).collect();

        DayFour { passports }
    }

    pub fn default() -> Self {
        DayFour::new("data/4a.txt")
    }

    pub fn solve_one(&self) -> usize {
        self.passports.iter().filter(|x| x.complete()).count()
    }
}

struct Passport {
    byr: Option<String>,
    iyr: Option<String>,
    eyr: Option<String>,
    hgt: Option<String>,
    hcl: Option<String>,
    ecl: Option<String>,
    pid: Option<String>,
    _cid: Option<String>,
}

impl Passport {
    fn new(definition: &str) -> Passport {
        let mut fields: HashMap<String, String> = HashMap::new();
        for s in definition
            .to_string()
            .split(|c| c == ' ' || c == '\n')
            .map(str::to_string)
            .collect::<Vec<String>>()
            .iter()
        {
            let parts: Vec<&str> = s.split(':').collect();
            fields.insert(parts[0].to_string(), parts[1].to_string());
        }

        Passport {
            byr: fields.get("byr").map(String::from),
            iyr: fields.get("iyr").map(String::from),
            eyr: fields.get("eyr").map(String::from),
            hgt: fields.get("hgt").map(String::from),
            hcl: fields.get("hcl").map(String::from),
            ecl: fields.get("ecl").map(String::from),
            pid: fields.get("pid").map(String::from),
            _cid: fields.get("cid").map(String::from),
        }
    }

    // Are all the necessary fields present (even if not valid?)
    fn complete(&self) -> bool {
        // All fields except 'cid'
        self.byr.is_some()
            && self.iyr.is_some()
            && self.eyr.is_some()
            && self.hgt.is_some()
            && self.hcl.is_some()
            && self.ecl.is_some()
            && self.pid.is_some()
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
