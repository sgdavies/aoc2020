use crate::file_to_vec;

pub(crate) fn part_one(filename: &str) -> u32 {
    let instructions = file_to_vec(filename);

    let mut x = 0i32;
    let mut y = 0i32;
    let mut facing = 90i32;

    for instruction in instructions.iter() {
        let code = instruction.chars().next().unwrap();
        let value = (&instruction[1..]).parse::<i32>().unwrap();
        match code {
            'N' => y += value,
            'E' => x += value,
            'S' => y -= value,
            'W' => x -= value,
            'R' => {
                facing += value;
                facing %= 360;
            }
            'F' => match facing {
                0 => y += value,
                90 => x += value,
                180 => y -= value,
                270 => x -= value,
                other => panic!("Unexpected direction {:}", other),
            },
            _ => panic!("Unexpected code/val {:}-{:}", code, value),
        }
    }
    (x.abs() + y.abs()) as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        assert!(part_one("data/12_example.txt") == 25);
    }
}
