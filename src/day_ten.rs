use crate::file_to_vec;

pub(crate) fn solve_10a(filename: &str) -> u32 {
    let (one_runs, threes) = get_gaps(filename);
    one_runs.iter().sum::<u32>() * threes
}

pub(crate) fn solve_10b(filename: &str) -> u64 {
    let (one_runs, _) = get_gaps(filename);

    // Can split at any gap of 3 - there's only one way
    // to cross the gap.  So each string of gaps-of-one
    // has a variety of ways through - the only requirement
    // is you hit the final number in the group (before the
    // gap of three).
    // Calculated these by hand, after seeing max number of
    // consecutive ones in my input was small.
    one_runs.iter().fold(1, |acc, x| {
        acc * match x {
            0 => 1, // consecutive 3s
            1 => 1,
            2 => 2,
            3 => 4,
            4 => 7,
            too_big => panic!(
                "Unexpectedly large gap ({:}) - calculate combinations for this",
                too_big
            ),
        }
    })
}

fn get_gaps(filename: &str) -> (Vec<u32>, u32) {
    let mut adaptors: Vec<u32> = file_to_vec(filename)
        .iter()
        .map(|s| s.parse::<u32>().unwrap())
        .collect();
    adaptors.sort_unstable();
    adaptors.push(adaptors[adaptors.len() - 1] + 3);

    let mut one_runs: Vec<u32> = vec![0];
    let mut threes = 0u32;

    for i in 0..adaptors.len() {
        let other = match i {
            0 => 0, // Outlet
            _ => adaptors[i - 1],
        };
        match adaptors[i] - other {
            x if x == 1 => *one_runs.last_mut().unwrap() += 1,
            x if x == 3 => {
                threes += 1;
                one_runs.push(0);
            }
            // Input is friendly - there are only gaps of 1 or 3
            oops => panic!("Unexpected gap length: {:} (at index {:})", oops, i),
        };
    }

    println!("{:?} {:}", one_runs, threes);
    (one_runs, threes)
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_one() {
        assert!(solve_10a("data/10_ex1.txt") == 35);
        assert!(solve_10a("data/10_ex2.txt") == 220);
    }

    #[test]
    fn test_two() {
        assert!(solve_10b("data/10_ex1.txt") == 8);
        assert!(solve_10b("data/10_ex2.txt") == 19208);
    }
    // (0) 1 4 5 6 7 10 11 12 15 16 19 (22)
    //    1 3 111 3 11 3 1 33
}
