use crate::file_to_vec;
use std::collections::HashSet;

pub(crate) fn part_one(filename: &str) -> usize {
    // (coords) -> (active, neighbours' coords)
    let mut grid: HashSet<(isize, isize, isize)> = HashSet::new();

    for (y, line) in file_to_vec(filename).iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            match c {
                '.' => {}
                '#' => {
                    grid.insert((x as isize, y as isize, 0));
                }
                _ => panic!("Unexpected input char: {:}", c),
            }
        }
    }

    for _ in 0..6 {
        let mut next_grid: HashSet<(isize, isize, isize)> = HashSet::new();

        // Only need to check coords which are touching an active cell - oh, and the active cells themselves!
        let coords_to_consider: HashSet<(isize, isize, isize)> =
            grid.iter().fold(HashSet::new(), |mut acc, c| {
                acc.insert(*c);
                acc.union(&get_neighbours(*c)).copied().collect()
            });

        for coords in coords_to_consider.iter() {
            let active_neighbours = get_neighbours(*coords)
                .iter()
                .filter(|c| grid.contains(c))
                .count();

            let new_active = match grid.contains(coords) {
                true => active_neighbours == 2 || active_neighbours == 3,
                false => active_neighbours == 3,
            };

            if new_active {
                next_grid.insert(*coords);
            }
        }

        grid = next_grid;
    }

    grid.iter().count()
}

fn get_neighbours(coords: (isize, isize, isize)) -> HashSet<(isize, isize, isize)> {
    let mut neighbours: HashSet<(isize, isize, isize)> = HashSet::new();
    let x = coords.0;
    let y = coords.1;
    let z = coords.2;

    // ring of 8 around you, and up & down
    for nz in z - 1..z + 1 + 1 {
        for (nx, ny) in &[
            (x - 1, y - 1),
            (x, y - 1),
            (x + 1, y - 1),
            (x - 1, y),
            (x + 1, y),
            (x - 1, y + 1),
            (x, y + 1),
            (x + 1, y + 1),
        ] {
            neighbours.insert((*nx, *ny, nz));
        }
    }

    // And 1 above, and 1 below
    neighbours.insert((x, y, z - 1));
    neighbours.insert((x, y, z + 1));

    neighbours
}

pub(crate) fn part_two(filename: &str) -> usize {
    // (coords) -> (active, neighbours' coords)
    let mut grid: HashSet<(isize, isize, isize, isize)> = HashSet::new();

    for (y, line) in file_to_vec(filename).iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            match c {
                '.' => {}
                '#' => {
                    grid.insert((x as isize, y as isize, 0, 0));
                }
                _ => panic!("Unexpected input char: {:}", c),
            }
        }
    }

    for _ in 0..6 {
        let mut next_grid: HashSet<(isize, isize, isize, isize)> = HashSet::new();

        // Only need to check coords which are touching an active cell - oh, and the active cells themselves!
        let coords_to_consider: HashSet<(isize, isize, isize, isize)> =
            grid.iter().fold(HashSet::new(), |mut acc, c| {
                acc.insert(*c);
                acc.union(&get_neighbours_4(*c)).copied().collect()
            });

        for coords in coords_to_consider.iter() {
            let active_neighbours = get_neighbours_4(*coords)
                .iter()
                .filter(|c| grid.contains(c))
                .count();

            let new_active = match grid.contains(coords) {
                true => active_neighbours == 2 || active_neighbours == 3,
                false => active_neighbours == 3,
            };

            if new_active {
                next_grid.insert(*coords);
            }
        }

        grid = next_grid;
    }

    grid.iter().count()
}

fn get_neighbours_4(coords: (isize, isize, isize, isize)) -> HashSet<(isize, isize, isize, isize)> {
    let mut neighbours: HashSet<(isize, isize, isize, isize)> = HashSet::new();
    let x = coords.0;
    let y = coords.1;
    let z = coords.2;
    let w = coords.3;

    let range = [-1, 0, 1];

    for &dx in &range {
        for &dy in &range {
            for &dz in &range {
                for &dw in &range {
                    if !(dx == 0 && dy == 0 && dz == 0 && dw == 0) {
                        neighbours.insert((x + dx, y + dy, z + dz, w + dw));
                    }
                }
            }
        }
    }

    neighbours
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_neighbours() {
        assert!(get_neighbours((0, 0, 0)).len() == 26);
        assert!(get_neighbours((1, 2, 3)).contains(&(2, 2, 2)));
        assert!(get_neighbours((1, 2, 3)).contains(&(0, 2, 3)));
    }

    #[test]
    fn test_one() {
        assert!(part_one("data/17_example.txt") == 112);
    }

    #[test]
    fn test_neighbours_4() {
        assert!(get_neighbours_4((0, 0, 0, 0)).len() == 80);
        assert!(get_neighbours_4((1, 2, 3, 4)).contains(&(2, 2, 3, 3)));
        assert!(get_neighbours_4((1, 2, 3, 4)).contains(&(0, 2, 3, 4)));
    }
}
