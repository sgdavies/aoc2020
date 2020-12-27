use crate::file_to_vec;
use std::collections::VecDeque;

pub(crate) fn part_one(deck_one: &str, deck_two: &str) -> i32 {
    let mut deck_one: VecDeque<i32> = file_to_vec(deck_one).iter().map(|s| s.parse::<i32>().unwrap()).collect();
    let mut deck_two: VecDeque<i32> = file_to_vec(deck_two).iter().map(|s| s.parse::<i32>().unwrap()).collect();

    while !deck_one.is_empty() && !deck_two.is_empty() {
        let one = deck_one.pop_front().unwrap();
        let two = deck_two.pop_front().unwrap();

        match one - two {
            x if x > 0 => {
                deck_one.push_back(one);
                deck_one.push_back(two);
            },
            x if x < 0 => {
                deck_two.push_back(two);
                deck_two.push_back(one);
            },
            _ => panic!("Cards must all be unique! {:} - {:}", one, two),
        }
    }

    let winning_deck = match deck_one.len() {
        0 => deck_two,
        _ => deck_one,
    };

    let len = winning_deck.len();
    winning_deck.iter().enumerate().fold(0, |acc, (i, v)| acc + ((len - i) as i32) * v)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_one() {
        assert!(part_one("data/22_ex1.txt", "data/22_ex2.txt") == 306);
    }
}