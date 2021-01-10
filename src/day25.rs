const SUBJECT: u64 = 7;
const CEIL: u64 = 20201227;

pub(crate) fn part_one(card_pub: u64, door_pub: u64) -> u64 {
    let door_loop = find_loop(door_pub);
    // println!("Door loop {:}", door_loop);

    transform(card_pub, door_loop)
}

fn transform(subject: u64, loops: u64) -> u64 {
    let mut val = 1;
    for _ in 0..loops {
        val *= subject;
        val %= CEIL;
    }

    val
}

fn find_loop(pub_key: u64) -> u64 {
    let mut loops = 1;
    let mut val = 1;

    loop {
        val *= SUBJECT;
        val %= CEIL;
        if val == pub_key {
            return loops;
        }

        loops += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transform() {
        assert!(transform(SUBJECT, 8) == 5764801);
        assert!(transform(SUBJECT, 11) == 17807724);
    }

    #[test]
    fn test_loop() {
        assert!(find_loop(5764801) == 8);
        assert!(find_loop(17807724) == 11);
    }

    #[test]
    fn test_one() {
        assert!(part_one(5764801, 17807724) == 14897079);
        assert!(part_one(17807724, 5764801) == 14897079);
    }
}
