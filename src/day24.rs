use crate::file_to_vec;
use std::collections::{HashMap, HashSet};

pub(crate) fn part_one(input: &str) -> usize {
    build_floor(input).values().filter(|x| **x).count()
}

pub(crate) fn part_two(input: &str) -> usize {
    const DAYS: usize = 100;
    let mut floor = build_floor(input);
    let mut neighbours = Neighbours::new();

    for _i in 0..DAYS {
        let mut new_floor: HashMap<(i32, i32), bool> = HashMap::new();

        // Need to consider all black tiles, and all tiles touching a black tile
        let black_tiles: HashSet<(i32, i32)> = floor
            .iter()
            .filter(|(_k, v)| **v)
            .map(|(coords, _v)| *coords)
            .collect();
        let possible_tiles: HashSet<(i32, i32)> =
            black_tiles.iter().fold(black_tiles.clone(), |acc, coords| {
                acc.union(&neighbours.neighbours(coords))
                    .copied()
                    .collect()
            });

        for tile in possible_tiles.iter() {
            let black_neighbours = neighbours.neighbours(tile).iter().fold(0, |acc, neigh| {
                acc + match floor.get(neigh) {
                    Some(true) => 1,
                    _ => 0,
                }
            });

            match floor.get(tile) {
                Some(true) => {
                    // Black tile. 0 or 3+ black neighbours -> turn white
                    if black_neighbours == 1 || black_neighbours == 2 {
                        new_floor.insert(*tile, true);
                    }
                }
                _ => {
                    // White tile. 2 black neighbours -> turn black
                    if black_neighbours == 2 {
                        new_floor.insert(*tile, true);
                    }
                }
            }
        }

        floor = new_floor;
        // println!("Day {:}: {:}", _i + 1, floor.values().filter(|x| **x).count());
    }

    println!(
        "Cache: size {:} hit/miss {:}/{:}",
        neighbours.memo.iter().count(),
        neighbours.hits,
        neighbours.misses
    );
    floor.values().filter(|x| **x).count()
}

fn build_floor(input: &str) -> HashMap<(i32, i32), bool> {
    let tiles = file_to_vec(input);
    let tiles = tiles.iter().map(|s| coordinates(s));

    let mut floor: HashMap<(i32, i32), bool> = HashMap::new();
    for coords in tiles {
        let tile = floor.entry(coords).or_insert(false);
        *tile = !*tile;
    }

    floor
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
                    }
                    ('s', 'e') => {
                        x += 1;
                        y -= 1;
                    }
                    ('s', 'w') => y -= 1,
                    (a, b) => panic!("Can't understand direction {:} {:}", a, b),
                }
            }
        }
    }
    (x, y)
}

struct Neighbours {
    memo: HashMap<(i32, i32), HashSet<(i32, i32)>>,
    hits: usize,
    misses: usize,
}

impl Neighbours {
    fn new() -> Neighbours {
        Neighbours {
            memo: HashMap::new(),
            hits: 0,
            misses: 0,
        }
    }

    fn neighbours(&mut self, coords: &(i32, i32)) -> HashSet<(i32, i32)> {
        match self.memo.get(coords) {
            Some(neigh) => {
                self.hits += 1;
                neigh.clone()
            }
            None => {
                self.misses += 1;
                let neigh = Neighbours::calculate_neighbours(coords);
                self.memo.insert(*coords, neigh.clone());
                neigh
            }
        }
    }

    fn calculate_neighbours(coords: &(i32, i32)) -> HashSet<(i32, i32)> {
        let mut neigh: HashSet<(i32, i32)> = HashSet::new();
        let (x, y) = coords;
        neigh.insert((*x + 1, *y)); // E
        neigh.insert((*x + 1, *y - 1)); // SE
        neigh.insert((*x, *y - 1)); // SW
        neigh.insert((*x - 1, *y)); // W
        neigh.insert((*x - 1, *y + 1)); // NW
        neigh.insert((*x, *y + 1)); // NE

        neigh
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_one() {
        assert!(part_one("data/24_example.txt") == 10);
    }

    #[test]
    fn test_two() {
        assert!(part_two("data/24_example.txt") == 2208);
    }
}
