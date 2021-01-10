pub(crate) fn part_one(input: &str, rounds: usize) -> String {
    let start = input.chars().next().unwrap().to_digit(10).unwrap() as usize;
    let mut cups = array_of_nexts(input);

    crab_cups(start, &mut cups, rounds);

    let mut next = cups[1];
    let mut out = String::new();

    while next != 1 {
        out.push(std::char::from_digit(next as u32, 10).unwrap());
        next = cups[next];
    }

    out
}

pub(crate) fn part_two(input: &str, rounds: usize, n_cups: usize) -> u64 {
    let start = input.chars().next().unwrap().to_digit(10).unwrap() as usize;
    let mut cups = array_of_nexts(input);

    let mut next = cups.iter().max().unwrap() + 1;
    let last = cups
        .iter()
        .enumerate()
        .filter(|(_i, &x)| x == start)
        .map(|(i, _x)| i)
        .next()
        .unwrap();
    cups[last] = next;
    next += 1;
    while next <= n_cups {
        cups.push(next);
        next += 1;
    }
    cups.push(start);

    crab_cups(start, &mut cups, rounds);

    let next_after_one = cups[1];
    let the_one_after_that = cups[next_after_one];
    (next_after_one as u64) * (the_one_after_that as u64)
}

fn array_of_nexts(input: &str) -> Vec<usize> {
    // Convert the input string (like "389125467") into an array
    // where the entry at index i is the value of the cup next in
    // order.  E.g. at index 3 would be the value 8.
    // Index 0 exists but is unused (cup labels start at 1)
    let cups: Vec<usize> = input
        .chars()
        .map(|c| c.to_digit(10).unwrap() as usize)
        .collect();
    let len = cups.len();

    let mut next_cups: Vec<usize> = vec![0; cups.len() + 1];

    for i in 0..len {
        let next_i = (i + 1) % len;
        next_cups[cups[i]] = cups[next_i];
    }

    next_cups
}

pub(crate) fn crab_cups(start_cup: usize, cups: &mut Vec<usize>, rounds: usize) {
    let len = cups.len() - 1;
    let mut current = start_cup;

    for _i in 0..rounds {
        // The next three cups
        let one = cups[current];
        let two = cups[one];
        let three = cups[two];

        let next_current = cups[three];
        let destination = get_destination(current, one, two, three, len);
        let next = cups[destination];

        cups[destination] = one;
        // two and three are already in the right places
        cups[three] = next;
        cups[current] = next_current;

        current = next_current;
    }
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

    // #[test]
    fn _time() {
        use std::time::Instant;
        let all_turns = [30_000, 100_000]; // Linear on n-turns
        let all_sizes = [1_000_000]; // [1_000, 10_000, 100_000];
        for &turns in all_turns.iter() {
            for &size in all_sizes.iter() {
                let start = Instant::now();
                let _ = part_two("389254671", turns, size); //1_000_000);
                println!(
                    "{:} turns at {:} size\t{:}",
                    turns,
                    size,
                    start.elapsed().as_secs()
                );
            }
        }
        assert!(false); // Show test output in stdout
    }
}
