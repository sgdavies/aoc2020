use std::collections::LinkedList;

pub(crate) fn part_one(input: &str, rounds: usize) -> String {
    let cups: LinkedList<usize> = input
        .chars()
        .map(|c| c.to_digit(10).unwrap() as usize)
        .collect();

    let mut cups = crab_cups(cups, rounds);

    // Build a list running from the first entry _after_ 1 and round again
    while cups.front().unwrap() != &1 {
        let not_1 = cups.pop_front().unwrap();
        cups.push_back(not_1);
    }
    cups.pop_front(); // Remove the 1
    let mut out = String::new();
    for c in cups.iter() {
        out.push(std::char::from_digit(*c as u32, 10).unwrap());
    }

    out
}

pub(crate) fn part_two(input: &str, rounds: usize, n_cups: usize) -> usize {
    let mut cups: LinkedList<usize> = input
        .chars()
        .map(|c| c.to_digit(10).unwrap() as usize)
        .collect();
    for next in cups.len()..n_cups {
        cups.push_back(next + 1);
    }

    cups = crab_cups(cups, rounds);

    // Lazily assume 1 isn't right at the end of the array
    while cups.pop_front().unwrap() != 1 {}
    cups.pop_front().unwrap() * cups.pop_front().unwrap()
}

pub(crate) fn crab_cups(mut cups: LinkedList<usize>, rounds: usize) -> LinkedList<usize> {
    let len = cups.len();

    for _i in 0..rounds {
        let current = cups.pop_front().unwrap();
        let one = cups.pop_front().unwrap();
        let two = cups.pop_front().unwrap();
        let three = cups.pop_front().unwrap();
        let destination = get_destination(current, one, two, three, len);

        // Rust LinkedList doesn't have a find/index_of method :-(
        let dest_index = cups
            .iter()
            .enumerate()
            .fold(None, |a, (i, x)| match x {
                t if *t == destination => Some(i),
                _ => a,
            })
            .unwrap(); // Inefficient - always searches whole array.

        let mut back_cups = cups.split_off(dest_index + 1);
        cups.push_back(one);
        cups.push_back(two);
        cups.push_back(three);
        cups.append(&mut back_cups);

        cups.push_back(current);
    }

    cups
}

fn get_destination(current: usize, one: usize, two: usize, three: usize, len: usize) -> usize {
    let mut target = match current {
        1 => len,
        x => x - 1,
    };

    while target == one || target == two || target == three {
        target = match target {
            1 => len,
            x => x - 1,
        };
    }

    target
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rotate() {
        assert!(&part_one("23451", 0) == "2345");
    }

    #[test]
    fn test_destination() {
        assert!(get_destination(2, 4, 5, 6, 9) == 1);
        assert!(get_destination(1, 4, 5, 6, 9) == 9);
        assert!(get_destination(9, 4, 5, 6, 9) == 8);
        assert!(get_destination(7, 4, 5, 6, 9) == 3);
    }

    #[test]
    fn test_one() {
        assert!(&part_one("389125467", 1) == "54673289");
        assert!(&part_one("389125467", 10) == "92658374");
        assert!(&part_one("389125467", 100) == "67384529");
    }

    #[test]
    fn test_two() {
        assert!(part_two("389125467", 0, 1_000_000) == 2 * 5);
        assert!(part_two("389254671", 0, 1_000_000) == 10 * 11);
    }

    #[test]
    fn _time() {
        use std::time::Instant;
        let all_turns = [3_000]; //[1_000, 3_000, 10_000, 30_000];//, 3_000, 10_000]; // Linear on n-turns
        let all_sizes = [3_000, 10_000, 30_000, 100_000]; //[1_000, 10_000, 100_000];
        for &turns in all_turns.iter() {
            for &size in all_sizes.iter() {
                let start = Instant::now();
                let _ = part_two("389254671", turns, size); //1_000_000);
                println!("{:} turns at {:} size\t{:}", turns, size, start.elapsed().as_secs());
            }
        }
        assert!(false); // Show test output in stdout
    }
}
