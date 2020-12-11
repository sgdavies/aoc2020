use crate::file_to_vec;
use std::collections::VecDeque;

pub(crate) fn solve_9a(filename: &str, preamble_length: usize) -> u64 {
    let input: Vec<u64> = file_to_vec(filename)
        .iter()
        .map(|s| s.parse::<u64>().unwrap())
        .collect();

    input[find_invalid_number(&input, preamble_length)]
}

pub(crate) fn solve_9b(filename: &str, preamble_length: usize) -> u64 {
    let input: Vec<u64> = file_to_vec(filename)
        .iter()
        .map(|s| s.parse::<u64>().unwrap())
        .collect();
    let target = input[find_invalid_number(&input, preamble_length)];

    // Now to find the contiguous set.  Values are all positive, so use a concertina.
    let mut low = 0usize;
    let mut high = 1usize;
    let mut sum = input[low] + input[high];
    loop {
        match sum {
            x if x == target => {
                println!("{:} .. {:} = {:}", low, high, sum);
                let slice = &input[low..high + 1];
                return *slice.iter().min().unwrap() + *slice.iter().max().unwrap();
            }
            x if x < target => {
                // Too small - extend the top of the concertina
                high += 1;
                sum += input[high];
            }
            x if x > target => {
                // Too big - retract the bottom of the concertina
                sum -= input[low];
                low += 1;
            }
            _ => panic!("Not equal, lower, or higher...!?"),
        };
    }
}

fn find_invalid_number(input: &[u64], preamble_length: usize) -> usize {
    let mut possible_sums: VecDeque<Vec<u64>> = VecDeque::new();
    for i in 0..preamble_length {
        let mut sums: Vec<u64> = Vec::new();

        let lower: isize = i as isize - preamble_length as isize;
        let upper: isize = i as isize;
        for j in lower..upper {
            let sum: u64 = match j {
                index if index < 0 => 0,
                index => input[i] + input[index as usize],
            };
            sums.push(sum);
        }
        possible_sums.push_back(sums);
    }

    for i in preamble_length..input.len() {
        if !possible_sums
            .iter()
            .enumerate()
            .map(|(j, sums)| sums[sums.len() - j - 1..].iter().any(|x| *x == input[i]))
            .any(|x| x)
        {
            // println!("{:?} - {:?}", i, input[i]);
            // println!("{:?}", possible_sums);
            return i;
        }

        possible_sums.pop_front();
        let mut sums: Vec<u64> = Vec::new();
        for j in i - preamble_length..i {
            sums.push(input[i] + input[j]);
        }
        assert!(sums.len() == preamble_length);
        possible_sums.push_back(sums);
    }
    println!("{:?}", possible_sums);
    panic!("All numbers are possible");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_9a() {
        assert!(solve_9a("data/9_example.txt", 5) == 127);
    }

    #[test]
    fn test_9a_again() {
        let mut input: Vec<u64> = (1u64..26).collect();
        input.push(26);
        input.push(49);
        input.push(100);
        assert!(input[find_invalid_number(&input, 25)] == 100);
    }

    #[test]
    fn test_9b() {
        assert!(solve_9b("data/9_example.txt", 5) == 62);
    }
}
