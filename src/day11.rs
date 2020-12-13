use crate::file_to_vec;
use std::collections::HashMap;

// Day 11. Runs fine, but takes ~20s on each part (running on a
// Raspberry Pi).  Could be worth optimising.

struct Seats {
    seats: HashMap<(isize, isize), bool>,
    dim_x: isize,
    dim_y: isize,
}

pub(crate) fn part_one(filename: &str) -> usize {
    solve(filename, move_seats_one)
}

pub(crate) fn part_two(filename: &str) -> usize {
    solve(filename, move_seats_two)
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
        dim_x: dim_x as isize,
        dim_y: dim_y as isize,
    };

    while move_seats(&mut seats) {}
    let answer = seats.seats.values().filter(|b| **b).count();
    // println!("Iterations: {:}", answer);
    answer
}

// Return true if there were any seat changes, otherwise false
fn move_seats_one(seats: &mut Seats) -> bool {
    move_seats(seats, adjacent_one, 4)
}

// Return true if there were any seat changes, otherwise false
fn move_seats_two(seats: &mut Seats) -> bool {
    move_seats(seats, visible, 5)
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
    // _pretty_print(&seats);
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

fn visible(x: isize, y: isize, old_seats: &Seats) -> u32 {
    // Returns the number of directions in which you can see an
    // occupied chair, stopping at the first chair (occupied or empty)
    let direction_steps = [
        (-1, -1),
        (0, -1),
        (1, -1),
        (-1, 0),
        (1, 0),
        (-1, 1),
        (0, 1),
        (1, 1),
    ];
    direction_steps.iter().fold(0, |acc, (sx, sy)| {
        let mut next_x = x + sx;
        let mut next_y = y + sy;
        let mut seen = false;
        while 0 <= next_x && next_x < old_seats.dim_x && 0 <= next_y && next_y < old_seats.dim_y {
            if let Some(occupied) = old_seats.seats.get(&(next_x, next_y)) {
                seen |= occupied;
                break;
            }

            next_x += sx;
            next_y += sy;
        }

        match seen {
            true => acc + 1,
            false => acc,
        }
    })
}

fn _pretty_print(seats: &Seats) {
    for y in 0..seats.dim_y {
        let mut s: String = String::new();
        for x in 0..seats.dim_x {
            s.push(match seats.seats.get(&(x, y)) {
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
        assert!(part_one("data/11_example.txt") == 37);
    }

    #[test]
    fn test_two() {
        assert!(part_two("data/11_example.txt") == 26);
    }
}
