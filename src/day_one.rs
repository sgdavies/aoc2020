pub mod day_one {
    use crate::{file_to_vec, Solve};

    const TARGET: i32 = 2020;
    pub struct DayOne { data: Vec<i32> }

    impl DayOne {
        fn new(filename: &str) -> Self {
            DayOne { data: file_to_vec(filename)
                .iter()
                .map(|val| val.parse::<i32>().unwrap())
                .collect(), }
        }

        fn solve_one(&self) -> i32 {
            for (i, one) in self.data.iter().enumerate() {
                for two in &(*self.data)[i + 1..] {
                    if one + two == TARGET {
                        println!("Part one solved! {} {} -> {}", one, two, one * two);
                        return one * two;
                    }
                }
            }

            panic!("No solution found for part one :-(");
        }

        fn solve_two(&self) -> i32 {
            for (i, one) in self.data.iter().enumerate() {
                for (j, two) in (&(*self.data)[i + 1..]).iter().enumerate() {
                    for three in &(*self.data)[j + 1..] {
                        if one + two + three == TARGET {
                            println!(
                                "Part two solved! {} {} {} -> {}",
                                one,
                                two,
                                three,
                                one * two * three
                            );
                            return one * two * three;
                        }
                    }
                }
            }

            panic!("No solution found for part two :-(");
        }
    }

    impl Solve for DayOne {
        fn solve(_args: &Vec<String>) {
            let day = DayOne::new("data/1a.txt");
            day.solve_one();
            day.solve_two();
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test() {
            let day = DayOne::new("data/1a_example.txt");

            assert!(day.solve_one() == 514579);
            assert!(day.solve_two() == 241861950);
        }
    }
}
