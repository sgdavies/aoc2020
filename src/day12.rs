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
            'L' => {
                facing += 360 - value;
                facing %= 360;
            }
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

pub(crate) fn part_two(filename: &str) -> u32 {
    let instructions = file_to_vec(filename);

    let mut x = 0i32;
    let mut y = 0i32;
    let mut waypoint = Waypoint { x: 10, y: 1 };

    for instruction in instructions.iter() {
        let code = instruction.chars().next().unwrap();
        let value = (&instruction[1..]).parse::<i32>().unwrap();
        match code {
            'N' => waypoint.y += value,
            'E' => waypoint.x += value,
            'S' => waypoint.y -= value,
            'W' => waypoint.x -= value,
            'L' => waypoint.rotate_left(value),
            'R' => waypoint.rotate_right(value),
            'F' => {
                x += value * waypoint.x;
                y += value * waypoint.y;
            }
            _ => panic!("Unexpected code/val {:}-{:}", code, value),
        };
        // println!("ship {:}, {:}, waypoint {:}, {:}", x, y, waypoint.x, waypoint.y);
    }
    (x.abs() + y.abs()) as u32
}

struct Waypoint {
    x: i32,
    y: i32,
}

impl Waypoint {
    fn rotate_left(&mut self, degrees: i32) {
        self.rotate_right(360 - degrees);
    }

    fn rotate_right(&mut self, degrees: i32) {
        match degrees {
            90 => {
                let old_x = self.x;
                self.x = self.y;
                self.y = -old_x;
            }
            180 => {
                self.x = -self.x;
                self.y = -self.y;
            }
            270 => {
                let old_x = self.x;
                self.x = -self.y;
                self.y = old_x;
            }
            other => panic!("Unexpected direction {:}", other),
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        assert!(part_one("data/12_example.txt") == 25);
    }

    #[test]
    fn test_part_two() {
        assert!(part_two("data/12_example.txt") == 286);
    }
}
