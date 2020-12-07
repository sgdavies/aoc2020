use std::collections::HashMap;
use std::fs;

use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref HEIGHT_RE: Regex = Regex::new(r"^(\d+)(in|cm)$").unwrap();
}

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

    pub fn solve_two(&self) -> usize {
        self.passports.iter().filter(|x| x.valid()).count()
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

    // Valid, according to all the rules
    fn valid(&self) -> bool {
        check_byr(self.byr.as_ref())
            && check_iyr(self.iyr.as_ref())
            && check_eyr(self.eyr.as_ref())
            && check_hgt(self.hgt.as_ref())
            && check_hcl(self.hcl.as_ref())
            && check_ecl(self.ecl.as_ref())
            && check_pid(self.pid.as_ref())
    }
}

fn check_year(year: Option<&String>, earliest: u32, latest: u32) -> bool {
    match year {
        Some(year) => check_int_str(year, earliest, latest),
        None => false,
    }
}

fn check_int_str(val: &str, lower: u32, upper: u32) -> bool {
    match val.parse::<u32>() {
        Ok(val) => lower <= val && val <= upper,
        Err(_) => false,
    }
}

fn check_byr(byr: Option<&String>) -> bool {
    check_year(byr, 1920, 2002)
}

fn check_iyr(iyr: Option<&String>) -> bool {
    check_year(iyr, 2010, 2020)
}

fn check_eyr(eyr: Option<&String>) -> bool {
    check_year(eyr, 2020, 2030)
}

fn check_hgt(hgt: Option<&String>) -> bool {
    match hgt {
        Some(hgt) => match HEIGHT_RE.captures(hgt) {
            Some(cap) => match &cap[2] {
                "in" => check_int_str(&cap[1].to_string(), 59, 76),
                "cm" => check_int_str(&cap[1].to_string(), 150, 193),
                other => panic!("{:?}", other),
            },
            None => false,
        },
        None => false,
    }
}

fn check_hcl(hcl: Option<&String>) -> bool {
    match hcl {
        Some(_hcl) => false, // TODO
        None => false,
    }
}

fn check_ecl(ecl: Option<&String>) -> bool {
    match ecl {
        Some(_ecl) => false, // TODO
        None => false,
    }
}

fn check_pid(pid: Option<&String>) -> bool {
    match pid {
        Some(_pid) => false, // TODO
        None => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_one() {
        let four = DayFour::new("data/4a_example.txt");
        assert!(four.solve_one() == 2);
    }

    #[test]
    fn test_two_invalid() {
        let four = DayFour::new("data/4b_invalid.txt");
        assert!(four.solve_two() == 0);
    }

    #[test]
    fn test_two_valid() {
        let four = DayFour::new("data/4b_valid.txt");
        assert!(four.solve_two() == 4);
    }

    #[test]
    fn test_byr() {
        assert!(check_byr(Some(&"1920".to_string())));
        assert!(check_byr(Some(&"2002".to_string())));
        assert!(!check_byr(Some(&"1919".to_string())));
        assert!(!check_byr(Some(&"2003".to_string())));
        assert!(!check_byr(Some(&"192".to_string())));
        assert!(!check_byr(Some(&"abc".to_string())));
    }

    #[test]
    fn test_iyr() {
        assert!(check_iyr(Some(&"2010".to_string())));
        assert!(check_iyr(Some(&"2020".to_string())));
        assert!(!check_iyr(Some(&"2009".to_string())));
        assert!(!check_iyr(Some(&"2021".to_string())));
    }

    #[test]
    fn test_eyr() {
        assert!(check_eyr(Some(&"2020".to_string())));
        assert!(check_eyr(Some(&"2030".to_string())));
        assert!(!check_eyr(Some(&"2019".to_string())));
        assert!(!check_eyr(Some(&"2031".to_string())));
    }

    #[test]
    fn test_hgt() {
        assert!(check_hgt(Some(&"60in".to_string())));
        assert!(check_hgt(Some(&"190cm".to_string())));
        assert!(!check_hgt(Some(&"190in".to_string())));
        assert!(!check_hgt(Some(&"190".to_string())));
    }

    #[test]
    fn test_hcl() {
        assert!(check_hcl(Some(&"#123abc".to_string())));
        assert!(!check_hcl(Some(&"#123abz".to_string())));
        assert!(!check_hcl(Some(&"123abc".to_string())));
    }

    #[test]
    fn test_ecl() {
        assert!(check_ecl(Some(&"amb".to_string())));
        assert!(check_ecl(Some(&"blu".to_string())));
        assert!(check_ecl(Some(&"brn".to_string())));
        assert!(check_ecl(Some(&"gry".to_string())));
        assert!(check_ecl(Some(&"grn".to_string())));
        assert!(check_ecl(Some(&"hzl".to_string())));
        assert!(check_ecl(Some(&"oth".to_string())));

        assert!(check_ecl(Some(&"".to_string())));
        assert!(check_ecl(Some(&"wat".to_string())));
    }

    #[test]
    fn test_pid() {
        assert!(check_pid(Some(&"000000001".to_string())));
        assert!(check_pid(Some(&"100000002".to_string())));

        assert!(!check_pid(Some(&"10000002".to_string())));
        assert!(!check_pid(Some(&"1a0000002".to_string())));
        assert!(!check_pid(Some(&"100 00002".to_string())));
        assert!(!check_pid(Some(&"0123456789".to_string())));
        assert!(!check_pid(Some(&"1234567890".to_string())));
    }
}
