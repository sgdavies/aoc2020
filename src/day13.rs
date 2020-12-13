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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        assert!(part_one("data/13_example.txt") == 295);
    }
}
