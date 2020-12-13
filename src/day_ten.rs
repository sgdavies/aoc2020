use crate::file_to_vec;
use std::collections::HashMap;

pub(crate) fn solve_10a(filename: &str) -> u32 {
    let mut adaptors: Vec<u32> = file_to_vec(filename)
        .iter()
        .map(|s| s.parse::<u32>().unwrap())
        .collect();
    adaptors.sort_unstable();
    adaptors.push(adaptors[adaptors.len() - 1] + 3);

    // let mut ones = 0u32;
    // let mut threes = 0u32;
    let mut gaps: HashMap<u32, u32> = HashMap::new();

    for i in 0..adaptors.len() {
        let other = match i {
            0 => 0, // Outlet
            _ => adaptors[i - 1],
        };
        *gaps.entry(adaptors[i] - other).or_insert(0) += 1;
        // match adaptors[i] - other {
        //     x if x == 1 => ones += 1,
        //     x if x ==3 => threes += 1,
        //     _ => (),
        // };
    }

    // println!("{:}, {:}", ones, threes);
    println!("{:?}", gaps);
    gaps.get(&1).unwrap() * gaps.get(&3).unwrap()
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_one() {
        assert!(solve_10a("data/10_ex1.txt") == 35);
        assert!(solve_10a("data/10_ex2.txt") == 220);
    }

    // (0) 1 4 5 6 7 10 11 12 15 16 19 (22)
    //    1 3 111 3 11 3 1 33
}
