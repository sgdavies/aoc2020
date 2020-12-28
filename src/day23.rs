pub(crate) fn part_one(input: &str, rounds: usize) -> String {
    let cups: Vec<usize> = input.chars().map(|c| c.to_digit(10).unwrap() as usize).collect();

    let cups = crab_cups(cups, rounds);

    // Rotate so 1 is at the start
    let ix_1 = cups.iter().position(|&x| x == 1).unwrap();
    let (last, first) = cups.split_at(ix_1);
    let mut out = String::new();
    for c in first.iter() {
        match c {
            1 => {},
            _ => out.push(std::char::from_digit(*c as u32, 10).unwrap()),
        }
    } 
    for c in last.iter() {
        out.push(std::char::from_digit(*c as u32, 10).unwrap());
    } 
    out
}

pub(crate) fn crab_cups(mut cups: Vec<usize>, rounds: usize) -> Vec<usize> {
    let len = cups.len();

    for _i in 0..rounds {
        let current = cups[0];
        let (one, two, three) = (cups[1], cups[2], cups[3]);
        let destination = get_destination(current, one, two, three, len);
        // println!("Round {:} - c {:}, [{:} {:} {:}] -> {:}", _i, current, one, two, three, destination);

        let mut next_cups: Vec<usize> = Vec::new();
        for &c in cups[4..].iter() {
            next_cups.push(c);
            if c == destination {
                next_cups.push(one);
                next_cups.push(two);
                next_cups.push(three);
            }
        }
        next_cups.push(current);

        cups = next_cups;
        // println!("  next {:?}", cups);
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
    fn test_game() {
        assert!(&part_one("389125467", 1) == "54673289");
        assert!(&part_one("389125467", 10) == "92658374");
        assert!(&part_one("389125467", 100) == "67384529");
    }
}