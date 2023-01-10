use crate::file_to_vec;

pub struct DayThree {
    data: Vec<Vec<u8>>,
    width: usize,
}

impl DayThree {
    fn new(filename: &str) -> Self {
        let data: Vec<Vec<u8>> = file_to_vec(filename)
            .iter()
            .map(|s| s.as_bytes().to_vec())
            .collect();
        let width = data[0].len();
        DayThree { data, width }
    }

    pub fn default() -> Self {
        DayThree::new("data/3a.txt")
    }

    pub fn solve_one(&self) -> u64 {
        self.solve_for_slope(3, 1)
    }

    pub fn solve_two(&self) -> u64 {
        let slopes = vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];

        slopes
            .iter()
            .fold(1, |acc, (x, y)| acc * self.solve_for_slope(*x, *y))
    }

    fn solve_for_slope(&self, xstep: usize, ystep: usize) -> u64 {
        self.data
            .iter()
            .step_by(ystep)
            .enumerate()
            .fold(0, |acc, (i, row)| {
                acc + match row[(i as usize) * xstep % self.width] as char {
                    '#' => 1,
                    '.' => 0,
                    c => panic!("Unexpected char: {:?}", c),
                }
            })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let day_three = DayThree::new("data/3a_example.txt");
        assert!(day_three.solve_one() == 7);
    }

    #[test]
    fn test_two() {
        let day_three = DayThree::new("data/3a_example.txt");
        assert!(day_three.solve_two() == 336);
    }
}
