pub mod day_one {
    use crate::{file_to_vec, Solve, Test};

    const TARGET: i32 = 2020;
    pub struct DayOne {}

    impl DayOne {
        fn get_data(&self, filename: &str) -> Vec<i32> {
            file_to_vec(filename)
                .iter()
                .map(|val| val.parse::<i32>().unwrap())
                .collect()
        }

        fn solve_one(&self, data: &Vec<i32>) -> i32 {
            for (i, one) in data.iter().enumerate() {
                for two in &(*data)[i + 1..] {
                    if one + two == TARGET {
                        println!("Part one solved! {} {} -> {}", one, two, one * two);
                        return one * two;
                    }
                }
            }

            panic!("No solution found for part one :-(");
        }

        fn solve_two(&self, data: &Vec<i32>) -> i32 {
            for (i, one) in data.iter().enumerate() {
                for (j, two) in (&(*data)[i + 1..]).iter().enumerate() {
                    for three in &(*data)[j + 1..] {
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
        fn solve(&self, _args: &Vec<String>) {
            let data = self.get_data("data/1a.txt");
            self.solve_one(&data);
            self.solve_two(&data);
        }
    }

    impl Test for DayOne {
        fn test(&self, _args: &Vec<String>) {
            let data = self.get_data("data/1a_example.txt");

            println!("Data:\n{:?}", data);
            assert!(self.solve_one(&data) == 514579);
            assert!(self.solve_two(&data) == 241861950);
        }
    }
}
