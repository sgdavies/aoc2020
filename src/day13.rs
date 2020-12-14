use crate::file_to_vec;

pub(crate) fn part_one(filename: &str) -> u32 {
    let instructions = file_to_vec(filename);
    let now = instructions[0].parse::<u32>().unwrap();

    instructions[1]
        .split(',')
        .fold((u32::MAX, 0), |(min, answer), bus| {
            if let Ok(bus_time) = bus.parse::<u32>() {
                let time_to_wait = bus_time - now % bus_time;
                if time_to_wait < min {
                    (time_to_wait, time_to_wait * bus_time)
                } else {
                    (min, answer)
                }
            } else {
                (min, answer)
            }
        })
        .1
}

pub(crate) fn part_two(filename: &str) -> i64 {
    timestamp(&file_to_vec(filename)[1])
}

fn timestamp(input: &str) -> i64 {
    // input like "3,x,5,7"
    let mut multiplyer = 1i64;
    let mut diff = 0i64;
    for (i, s) in input.split(',').enumerate() {
        if let Ok(val) = s.parse::<i64>() {
            match i {
                0 => multiplyer *= val,
                i => {
                    for n in 0..val {
                        if (n * multiplyer - diff) % val == (val - (i as i64 % val)) {
                            diff = multiplyer * val - (n * multiplyer - diff);
                            multiplyer *= val;
                            // println!("i {:}, val {:}, n {:}, mult {:}, diff {:}", i, val, n, multiplyer, diff);
                            break; // next bus
                        }
                    }
                }
            }
        }
    }

    multiplyer - diff
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        assert!(part_one("data/13_example.txt") == 295);
    }

    #[test]
    fn test_timestamp() {
        assert!(timestamp("17,x,13,19") == 3417);
        assert!(timestamp("67,7,59,61") == 754018);
        assert!(timestamp("67,x,7,59,61") == 779210);
        assert!(timestamp("67,7,x,59,61") == 1261476);
        assert!(timestamp("1789,37,47,1889") == 1202161486);
    }

    #[test]
    fn test_part_two() {
        assert!(part_two("data/13_example.txt") == 1068781);
    }
}
