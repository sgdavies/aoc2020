use crate::file_to_vec;
use std::collections::HashMap;

pub(crate) fn solve_part_one(filename: &str) -> usize {
    let mut seats: HashMap<(isize, isize), bool> = HashMap::new();
    // file_to_vec(filename).iter().enumerate().map(|(y, s)| {
    //     s.chars().enumerate().map(|(x, c)| match c {
    //         '#' => {
    //             seats.insert((x, y), false);
    //         }
    //         '.' => {
    //             ();
    //         }
    //         unexpected => panic!("Unexpected input character: {:}", unexpected),
    //     })
    // });
    let input = file_to_vec(filename);
    // let (dim_x, dim_y) = (input[0].len(), input.len());
    for (y, s) in input.iter().enumerate() {
        for (x, c) in s.chars().enumerate() {
            match c {
                'L' => {
                    seats.insert((x as isize, y as isize), false);
                }
                '#' => {
                    seats.insert((x as isize, y as isize), true);
                }
                '.' => {}
                unexpected => panic!("Unexpected input character: {:}", unexpected),
            };
        }
    }

    loop {
        let (new_seats, changed) = move_seats(&seats);
        seats = new_seats;
        // println!("iter {:}, changed? {:}", i, changed);

        if !changed {
            return seats.values().filter(|b| **b).count();
        }
    }
}

// Return true if there were any seat changes, otherwise false
fn move_seats(seats: &HashMap<(isize, isize), bool>) -> (HashMap<(isize, isize), bool>, bool) {
    let mut new_seats: HashMap<(isize, isize), bool> = HashMap::new();
    let mut changed = false;

    for ((x, y), occupied) in seats.iter() {
        let adjacent_occupied = adjacent_indices(*x, *y).iter().fold(0, |acc, (ox, oy)| {
            acc + match seats.get(&(*ox, *oy)) {
                Some(true) => 1,
                _ => 0,
            }
        });
        let new_occupied = match (occupied, adjacent_occupied) {
            (true, adj) if adj >= 4 => {
                changed = true;
                false
            }
            (false, adj) if adj == 0 => {
                changed = true;
                true
            }
            (_, _) => *occupied,
        };
        // changed = new_occupied != *occupied;
        new_seats.insert((*x, *y), new_occupied);
    }

    (new_seats, changed)
}

fn adjacent_indices(x: isize, y: isize) -> Vec<(isize, isize)> {
    vec![
        (x - 1, y - 1),
        (x, y - 1),
        (x + 1, y - 1),
        (x - 1, y),
        (x + 1, y),
        (x - 1, y + 1),
        (x, y + 1),
        (x + 1, y + 1),
    ]
}

fn _pretty_print(seats: &HashMap<(isize, isize), bool>, dim_x: usize, dim_y: usize) {
    for y in 0..dim_y {
        let mut s: String = String::new();
        for x in 0..dim_x {
            s.push(match seats.get(&(x as isize, y as isize)) {
                Some(true) => '#',
                Some(false) => 'L',
                None => '.',
            });
        }
        println!("{}", s);
    }
    println!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_one() {
        assert!(solve_part_one("data/11_example.txt") == 37);
    }
}
