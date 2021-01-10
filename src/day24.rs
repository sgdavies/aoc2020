use crate::file_to_vec;
use std::collections::HashMap;

pub(crate) fn part_one(input: &str) -> usize {
    let tiles = file_to_vec(input);
    let tiles = tiles.iter().map(|s| coordinates(s));

    let mut floor: HashMap<(i32, i32), bool> = HashMap::new();
    for coords in tiles {
        let tile = floor.entry(coords).or_insert(false);
        *tile = !*tile;
    }

    floor.values().filter(|x| **x).count()
}

fn coordinates(tile: &str) -> (i32, i32) {
    // Hexagonal grid, using axial coordinate system
    // E-W = x-axis; NE-SW = y-axis; NW-SE = diagonal
    // E => +x, N => +y (so e.g. NW is -x, +y)
    let mut x = 0;
    let mut y = 0;
    let mut instructions = tile.chars();
    while let Some(c) = instructions.next() {
        match c {
            'e' => x += 1,
            'w' => x -= 1,
            n_s => {
                let e_w = instructions.next().unwrap();
                match (n_s, e_w) {
                    ('n', 'e') => y += 1,
                    ('n', 'w') => {
                        x -= 1;
                        y += 1;
                    },
                    ('s', 'e') => {
                        x += 1;
                        y -= 1;
                    },
                    ('s', 'w') => y -= 1,
                    (a, b) => panic!("Can't understand direction {:} {:}", a, b),
                }
            }
        }
    }
    (x, y)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_one() {
        assert!(part_one("data/24_example.txt") == 10);
    }
}