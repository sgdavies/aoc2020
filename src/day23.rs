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

pub(crate) fn part_two(input: &str, rounds: usize, n_cups: usize) -> u64 {
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
    (cups.pop_front().unwrap() as u64) * (cups.pop_front().unwrap() as u64)
}

pub(crate) fn crab_cups(mut cups: LinkedList<usize>, rounds: usize) -> LinkedList<usize> {
    let cups: Vec<usize> = cups.iter().map(|x| *x).collect();
    let len = cups.len();

    let mut current = cups[0];

    // Initialize the vec - empty.  We won't use the 0th index, but we do want a slot for every other item in the input list.
    let mut next_cups: Vec<usize> = vec![0];
    for _ in cups.iter() { next_cups.push(0); }

    for i in 0..len {
        let next_i = (i + 1) % len;
        next_cups[cups[i]] = cups[next_i];
    }

    for _i in 0..rounds {
        // The next three cups
        let one = next_cups[current];
        let two = next_cups[one];
        let three = next_cups[two];

        let next_current = next_cups[three];
        let destination = get_destination(current, one, two, three, len);
        let next = next_cups[destination];

        next_cups[destination] = one;
        // two and three are already in the right places
        next_cups[three] = next;
        next_cups[current] = next_current;

        current = next_current;
    }

    let mut cups = LinkedList::new();
    cups.push_back(1);
    let mut next = next_cups[1];
    loop {
        if next == 1 {
            break;
        }
        cups.push_back(next);
        next = next_cups[next];
    }

    // println!("{:?}", next_cups);
    // println!("{:?}", cups);
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
        assert!(&part_one("45123", 0) == "2345");
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
        let all_turns = [30_000, 100_000]; //[1_000, 3_000, 10_000, 30_000];//, 3_000, 10_000]; // Linear on n-turns
        let all_sizes = [1_000_000]; //[1_000, 10_000, 100_000];
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
