use crate::file_to_vec;
use std::collections::HashMap;

struct Seats {
    seats: HashMap<(isize, isize), bool>,
    dim_x: usize,
    dim_y: usize,
}

pub(crate) fn solve_part_one(filename: &str) -> usize {
    solve(filename, move_seats_one)
}

fn solve(filename: &str, move_seats: fn(&mut Seats) -> bool) -> usize {
    let mut seats: HashMap<(isize, isize), bool> = HashMap::new();
    let input = file_to_vec(filename);
    let (dim_x, dim_y) = (input[0].len(), input.len());
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

    let mut seats = Seats {
        seats,
        dim_x,
        dim_y,
    };

    while move_seats(&mut seats) {}
    seats.seats.values().filter(|b| **b).count()
}

// Return true if there were any seat changes, otherwise false
fn move_seats_one(seats: &mut Seats) -> bool {
    move_seats(seats, adjacent_one, 4)
}

fn move_seats(
    seats: &mut Seats,
    full_seats_i_care_about: fn(isize, isize, &Seats) -> u32,
    limit: u32,
) -> bool {
    let mut new_seats: HashMap<(isize, isize), bool> = HashMap::new();
    let mut changed = false;

    for ((x, y), occupied) in seats.seats.iter() {
        let adjacent_occupied = full_seats_i_care_about(*x, *y, &seats);
        let new_occupied = match (occupied, adjacent_occupied) {
            (true, adj) if adj >= limit => {
                changed = true;
                false
            }
            (false, adj) if adj == 0 => {
                changed = true;
                true
            }
            (_, _) => *occupied,
        };
        new_seats.insert((*x, *y), new_occupied);
    }

    seats.seats = new_seats;
    changed
}

fn adjacent_one(x: isize, y: isize, old_seats: &Seats) -> u32 {
    let old_seats = &old_seats.seats;
    adjacent_indices(x, y).iter().fold(0, |acc, (ox, oy)| {
        acc + match old_seats.get(&(*ox, *oy)) {
            Some(true) => 1,
            _ => 0,
        }
    })
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
