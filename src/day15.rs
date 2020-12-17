use std::collections::hash_map::Entry;
use std::collections::HashMap;

pub(crate) fn part_one(starters: &[usize], target: usize) -> usize {
    let mut seen = Seen::new(starters);
    let mut last = starters[starters.len() - 1];

    for turn in starters.len()..target {
        let next = match seen.seen.get(&last) {
            Some(v) => match v.len() {
                0 => panic!("Shouldn't have zero-length vec!"),
                1 => 0,
                n => v[n - 1] - v[n - 2],
            },
            None => 0,
        };

        seen.add(turn, next);
        // println!("t {:}, l {:}, n {:}\n{:?}", turn, last, next, seen.seen);
        last = next;
    }

    last
}

struct Seen {
    seen: HashMap<usize, Vec<usize>>,
}

impl Seen {
    fn new(starters: &[usize]) -> Seen {
        let mut seen = Seen {
            seen: HashMap::new(),
        };
        for (i, starter) in starters.iter().enumerate() {
            seen.add(i, *starter);
        }

        seen
    }

    fn add(&mut self, index: usize, number: usize) {
        match self.seen.entry(number) {
            Entry::Vacant(e) => {
                e.insert(vec![index]);
            }
            Entry::Occupied(mut e) => {
                e.get_mut().push(index);
            }
        }
    }
}

// 0 3 6 | 0 3 3 1 0 4 0
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_one() {
        assert!(part_one(&[0, 3, 6], 4) == 0);
        assert!(part_one(&[0, 3, 6], 5) == 3);
        assert!(part_one(&[0, 3, 6], 6) == 3);
        assert!(part_one(&[0, 3, 6], 7) == 1);
        assert!(part_one(&[0, 3, 6], 8) == 0);
        assert!(part_one(&[0, 3, 6], 9) == 4);
        assert!(part_one(&[0, 3, 6], 10) == 0);

        assert!(part_one(&[1, 3, 2], 2020) == 1);
        assert!(part_one(&[2, 1, 3], 2020) == 10);
        assert!(part_one(&[1, 2, 3], 2020) == 27);
        assert!(part_one(&[2, 3, 1], 2020) == 78);
        assert!(part_one(&[3, 2, 1], 2020) == 438);
        assert!(part_one(&[3, 1, 2], 2020) == 1836);
    }

    // #[test]
    fn _time() {
        use std::time::Instant;
        let all_turns = [100_000, 300_000, 1_000_000, 3_000_000, 10_000_000];
        for &turns in all_turns.iter() {
            let start = Instant::now();
            let _ = part_one(&[3, 1, 2], turns);
            println!("{:}\t{:}", turns, start.elapsed().as_secs());
        }
        assert!(false); // Show test output in stdout
    }
}
