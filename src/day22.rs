use crate::file_to_vec;
use std::collections::{HashSet, VecDeque};

pub(crate) fn part_one(deck_one: &str, deck_two: &str) -> i32 {
    let mut deck_one: VecDeque<i32> = file_to_vec(deck_one)
        .iter()
        .map(|s| s.parse::<i32>().unwrap())
        .collect();
    let mut deck_two: VecDeque<i32> = file_to_vec(deck_two)
        .iter()
        .map(|s| s.parse::<i32>().unwrap())
        .collect();

    while !deck_one.is_empty() && !deck_two.is_empty() {
        let one = deck_one.pop_front().unwrap();
        let two = deck_two.pop_front().unwrap();

        match one - two {
            x if x > 0 => {
                deck_one.push_back(one);
                deck_one.push_back(two);
            }
            x if x < 0 => {
                deck_two.push_back(two);
                deck_two.push_back(one);
            }
            _ => panic!("Cards must all be unique! {:} - {:}", one, two),
        }
    }

    let winning_deck = match deck_one.len() {
        0 => deck_two,
        _ => deck_one,
    };

    let len = winning_deck.len();
    winning_deck
        .iter()
        .enumerate()
        .fold(0, |acc, (i, v)| acc + ((len - i) as i32) * v)
}

#[derive(std::cmp::PartialEq)]
enum Winner {
    One,
    Two,
}

pub(crate) fn part_two(deck_one: &str, deck_two: &str) -> i32 {
    let mut deck_one: VecDeque<i32> = file_to_vec(deck_one)
        .iter()
        .map(|s| s.parse::<i32>().unwrap())
        .collect();
    let mut deck_two: VecDeque<i32> = file_to_vec(deck_two)
        .iter()
        .map(|s| s.parse::<i32>().unwrap())
        .collect();

    let winning_deck = match play_recursively(&mut deck_one, &mut deck_two) {
        Winner::One => deck_one,
        Winner::Two => deck_two,
    };

    let len = winning_deck.len();
    winning_deck
        .iter()
        .enumerate()
        .fold(0, |acc, (i, v)| acc + ((len - i) as i32) * v)
}

fn play_recursively(deck_one: &mut VecDeque<i32>, deck_two: &mut VecDeque<i32>) -> Winner {
    let mut seen: HashSet<(VecDeque<i32>, VecDeque<i32>)> = HashSet::new();

    while !deck_one.is_empty() && !deck_two.is_empty() {
        let key = (deck_one.clone(), deck_two.clone());
        if seen.contains(&key) {
            return Winner::One;
        } else {
            seen.insert(key);
        }

        let one = deck_one.pop_front().unwrap();
        let two = deck_two.pop_front().unwrap();

        let winner = if (one as usize) <= deck_one.len() && (two as usize) <= deck_two.len() {
            let mut r_deck_one: VecDeque<i32> = deck_one
                .iter()
                .enumerate()
                .filter(|(i, _v)| i <= &(one as usize - 1))
                .map(|(_i, v)| *v)
                .collect();
            let mut r_deck_two: VecDeque<i32> = deck_two
                .iter()
                .enumerate()
                .filter(|(i, _v)| i <= &(two as usize - 1))
                .map(|(_i, v)| *v)
                .collect();

            play_recursively(&mut r_deck_one, &mut r_deck_two)
        } else if one > two {
            Winner::One
        } else {
            Winner::Two
        };

        match winner {
            Winner::One => {
                deck_one.push_back(one);
                deck_one.push_back(two);
            }
            Winner::Two => {
                deck_two.push_back(two);
                deck_two.push_back(one);
            }
        }
    }

    if deck_one.is_empty() {
        Winner::Two
    } else {
        Winner::One
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_one() {
        assert!(part_one("data/22_ex1.txt", "data/22_ex2.txt") == 306);
    }

    #[test]
    fn test_two() {
        assert!(part_two("data/22_ex1.txt", "data/22_ex2.txt") == 291);
    }

    #[test]
    fn test_two_loop() {
        let mut deck_one: VecDeque<i32> = VecDeque::new();
        deck_one.push_back(43);
        deck_one.push_back(19);
        let mut deck_two: VecDeque<i32> = VecDeque::new();
        deck_two.push_back(2);
        deck_two.push_back(29);
        deck_two.push_back(14);
        assert!(play_recursively(&mut deck_one, &mut deck_two) == Winner::One);
    }
}
