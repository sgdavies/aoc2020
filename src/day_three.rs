use crate::file_to_vec;

pub struct DayThree {
    data: Vec<Vec<u8>>,
    width: usize,
}

impl DayThree {
    fn new(filename: &str) -> Self {
        let data: Vec<Vec<u8>> =  file_to_vec(filename).iter().map(|s| s.as_bytes().to_vec()).collect();
        let width = data[0].len();
        DayThree {
            data,
            width,
        }
    }

    pub fn default() -> Self {
        DayThree::new("data/3a.txt")
    }

    pub fn solve_one(&self) -> u32 {
        self.solve_for_slope(3, 1)
    }

    pub fn solve_two(&self) -> u32 {
        let slopes = vec![(1,1), (3,1), (5,1), (7,1), (1,2)];

        slopes.iter().fold(1, |acc, (x, y)| acc*self.solve_for_slope(*x,*y))
    }

    fn solve_for_slope(&self, xstep: usize, ystep: usize) -> u32 {
        let mut count = 0;
        let mut x = 0;
        for row in (0..self.data.len()).step_by(ystep) {
            if self.data[row][x] as char == '#' {
                count += 1;
            }

            x += xstep;
            x %= self.width;
        }
        count
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
