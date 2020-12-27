use lazy_static::lazy_static;
use std::collections::{HashMap, HashSet};
use std::sync::Mutex;
use std::{fmt, fs};

const TILE_WIDTH: usize = 10;

pub(crate) fn part_one(filename: &str) -> u64 {
    SolvedMosaic::solve(filename).part_one_value()
}

pub(crate) fn part_two(filename: &str) -> usize {
    SolvedMosaic::solve(filename).part_two_value()
}

struct SolvedMosaic {
    len: usize,
    tiles: Vec<Vec<Tile>>,
}

impl SolvedMosaic {
    fn solve(filename: &str) -> SolvedMosaic {
        let tiles: Vec<Tile> = fs::read_to_string(filename)
            .unwrap()
            .split("\n\n")
            .map(|s| Tile::from(s))
            .collect();

        let mut tile_to_tiles: HashMap<u64, Tile> = HashMap::new();
        let mut edge_to_tiles: HashMap<u16, HashSet<u64>> = HashMap::new();
        for tile in &tiles {
            tile_to_tiles.insert(tile.id, tile.clone());

            for edge in tile.possible_edges.iter() {
                edge_to_tiles
                    .entry(*edge)
                    .or_insert_with(HashSet::new)
                    .insert(tile.id);
            }
        }

        let mut mosaic = match tiles.len() {
            144 => Mosaic {
                len: 12,
                tile_to_tiles,
            },
            9 => Mosaic {
                len: 3,
                tile_to_tiles,
            },
            oops => panic!("Unexpected number of tiles: {:}", oops),
        };

        for tile in &tiles {
            if let Some(solution) = mosaic.solve_from(&tile) {
                return solution;
            }
        }

        panic!("Couldn't find a solution");
    }

    fn part_one_value(&self) -> u64 {
        self.tiles[0][0].id
            * self.tiles[0][self.len - 1].id
            * self.tiles[self.len - 1][0].id
            * self.tiles[self.len - 1][self.len - 1].id
    }

    fn part_two_value(&self) -> usize {
        // Stitch the tiles into a picture
        let mut picture: Vec<u128> = Vec::new();
        for tile_row in &self.tiles {
            // Trim off both top & bottom, and left & right
            for y in 1..(TILE_WIDTH - 1) {
                picture.push(tile_row.iter().enumerate().fold(0, |acc, (i, tile)| {
                    acc | (((tile.contents[y] as u128 >> 1) & 0b1111_1111)
                        << ((TILE_WIDTH - 2) * (self.len - 1 - i)))
                }));
            }
        }

        // (maybe print it, for fun!)
        for x in &picture {
            let width = self.len * (TILE_WIDTH - 2);
            println!("{:0width$b}", x, width = width);
        }

        let mut picture = Picture { pic: picture };

        // Find & count all the monsters - in all orientations of the picture
        let mut all_orientations: Vec<Picture> = Vec::new();
        all_orientations.push(picture.clone());
        for _ in 0..3 {
            picture.rotate();
            all_orientations.push(picture.clone());
        }
        picture.flip();
        all_orientations.push(picture.clone());
        for _ in 0..3 {
            picture.rotate();
            all_orientations.push(picture.clone());
        }
        assert!(all_orientations.len() == 8);

        // Count all the '#', take off the monsters => answer
        picture.count_bits()
            - all_orientations
                .iter()
                .fold(0, |max, pic| std::cmp::max(max, pic.count_monster_bits()))
    }
}

#[derive(Clone)]
struct Picture {
    pic: Vec<u128>,
}

impl Picture {
    fn count_bits(&self) -> usize {
        let x = self.pic.iter().fold(0, |acc, row| {
            let mut n_bits = 0;
            let mut number: u128 = *row;
            while number > 0 {
                n_bits += (number & 1) as usize;
                number >>= 1;
            }
            acc + n_bits
        });
        // println!("Bits in picture: {:}", x);
        x
    }

    fn count_monster_bits(&self) -> usize {
        // Find all the monsters in the picture
        // <                  # >
        // <#    ##    ##    ###>
        // < #  #  #  #  #  #   >
        let monster_parts: Vec<u128> = vec![
            0b00000000000000000010,
            0b10000110000110000111,
            0b01001001001001001000,
        ];
        let monster_length: usize = 20; // From tip to tail
        let bits_in_monster: usize = 15; // Could calculate this, but I just counted

        // Count the monsters
        let mut number_of_monsters = 0;

        let width = self.pic.len();
        for row_id in 0..width - 2 {
            // Monster is 3 lines long, so stop before the end
            for pos in 0..(width - monster_length) {
                if let true = (0..3).all(|i| { 
                    // Head, body & legs must all match
                    let mask = monster_parts[i] << (pos as u128);
                    (self.pic[row_id + i] & mask) == mask
                }) {
                    number_of_monsters += 1;
                    // Could skip forward a bit now, but no real need
                }
            }
        }

        // println!("Found {:} monsters", number_of_monsters);
        number_of_monsters * bits_in_monster
    }

    fn rotate(&mut self) {
        // Anti-clockwise
        let mut new_pic: Vec<u128> = Vec::new();

        let width = self.pic.len(); // It's a square
        for shift in 0..width {
            new_pic.push(Picture::get_right_128(
                &self.pic.iter().map(|&u| u >> shift).collect::<Vec<u128>>(),
                width,
            ));
        }

        self.pic = new_pic;
    }

    fn get_right_128(array: &[u128], width: usize) -> u128 {
        array
            .iter()
            .enumerate()
            .fold(0, |acc, (i, b)| acc | ((b & 1) << (width - 1 - i)))
    }

    fn flip(&mut self) {
        self.pic.reverse();
    }
}

struct Mosaic {
    len: usize,
    tile_to_tiles: HashMap<u64, Tile>,
}

impl Mosaic {
    fn solve_from(&mut self, tile: &Tile) -> Option<SolvedMosaic> {
        // Assume this is the top-left corner, and solve from here.
        // ("Top-left" is aribitrary - the picture can be any way up (flip/rotate))
        let mut tile = tile.clone();

        let mut rows: Vec<Vec<Tile>> = Vec::new();
        rows.push(Vec::new());
        rows[0].push(tile.clone());
        let mut remaining_tiles: HashSet<u64> = self
            .tile_to_tiles
            .keys()
            .filter(|&k| k != &tile.id)
            .copied()
            .collect();

        if let Some(answer) = self.complete_picture(&mut rows, &mut remaining_tiles) {
            return Some(answer);
        }

        // If this is a corner, then there are 2 (adjacent) edges that don't lead to solutions,
        // so also try this possible starting tile the other way around if we didn't find a solution the first time.
        tile.rotate();
        tile.rotate();
        let mut rows: Vec<Vec<Tile>> = Vec::new();
        rows.push(Vec::new());
        rows[0].push(tile.clone());
        let mut remaining_tiles: HashSet<u64> = self
            .tile_to_tiles
            .keys()
            .filter(|&k| k != &tile.id)
            .copied()
            .collect();

        self.complete_picture(&mut rows, &mut remaining_tiles)
    }

    fn complete_picture(
        &mut self,
        mut rows: &mut Vec<Vec<Tile>>,
        mut remaining_tiles: &mut HashSet<u64>,
    ) -> Option<SolvedMosaic> {
        if remaining_tiles.is_empty() {
            // The picture is complete!
            // Let's just double-check some stuff and then return the answer.
            let mut layout: String = String::new();
            for row in rows.iter() {
                for tile in row {
                    layout.push_str(&format!("{:} ", tile.id));
                }
                layout.push('\n');
            }
            rows.pop(); // Urgh, we pushed on one more empty row...
                        // println!("Solved!?\n{}", layout);
            assert!(rows.len() == self.len);
            assert!(rows.iter().all(|row| row.len() == self.len));

            return Some(SolvedMosaic {
                len: self.len,
                tiles: rows.to_vec(),
            });
        }

        let current_row_index: usize = rows.len() - 1; // May point to an empty vec
        let tile_to_left: Option<&Tile> = match rows[current_row_index].len() {
            0 => None,
            x => Some(&rows[current_row_index][x - 1]),
        };
        let tile_above: Option<&Tile> = match current_row_index {
            0 => None,
            y => Some(&rows[y - 1][rows[current_row_index].len()]), // The tile we're trying to insert is 1 right of the last tile
        };

        let candidate_tiles: HashSet<u64> = remaining_tiles
            .iter()
            .filter(|t_id| match tile_to_left {
                Some(left) => self
                    .tile_to_tiles
                    .get(t_id)
                    .unwrap()
                    .possible_edges
                    .contains(&left.right()),
                None => true,
            })
            .filter(|t_id| match tile_above {
                Some(above) => self
                    .tile_to_tiles
                    .get(t_id)
                    .unwrap()
                    .possible_edges
                    .contains(&above.bottom()),
                None => true,
            })
            .copied()
            .collect();

        // Candidates at least have edges that could match.  Now to rotate until they really do match.
        let candidate_tiles: HashSet<Tile> = match (tile_above, tile_to_left) {
            (Some(above), None) => candidate_tiles
                .iter()
                .map(|id| {
                    let mut tile = self.tile_to_tiles.get(id).unwrap().clone();
                    tile.turn_to_top(above.bottom());
                    tile
                })
                .collect(),
            (None, Some(left)) => candidate_tiles
                .iter()
                .map(|id| {
                    let mut tile = self.tile_to_tiles.get(id).unwrap().clone();
                    tile.turn_to_left(left.right());
                    tile
                })
                .collect(),
            (Some(above), Some(left)) => candidate_tiles
                .iter()
                .map(|id| {
                    let mut tile = self.tile_to_tiles.get(id).unwrap().clone();
                    tile.turn_to_top(above.bottom());
                    tile
                })
                .filter(|tile| tile.left() == left.right())
                .collect(),
            (None, None) => panic!("You told me one of these tiles matched!"),
        };

        for candidate in candidate_tiles {
            rows[current_row_index].push(candidate.clone());
            if rows[current_row_index].len() == self.len {
                rows.push(Vec::new());
            }
            remaining_tiles.remove(&candidate.id);

            if let Some(answer) = self.complete_picture(&mut rows, &mut remaining_tiles) {
                return Some(answer);
            }

            remaining_tiles.insert(candidate.id);
            if rows[current_row_index].len() == self.len {
                rows.pop();
            }
            rows[current_row_index].pop();
        }

        None
    }
}

#[derive(Clone, Eq)]
struct Tile {
    id: u64,

    // Will change as tile is flipped/rotated
    // top/bottom both read left-to-right; left, right both read top-to-bottom
    // This means if A stacks on B then A.bottom()==B.top() etc.
    contents: Vec<u16>,

    // All possible edges
    possible_edges: HashSet<u16>,
}

impl Tile {
    fn from(definition: &str) -> Tile {
        let mut lines = definition.lines();
        let id = lines.next().unwrap()[5..9].parse::<u64>().unwrap();

        // let chars: Vec<Vec<char>> = lines.map(|s| s.chars().collect()).collect();

        let contents: Vec<u16> = lines.map(|s| Tile::line_to_u16(s)).collect();
        let top = contents[0];
        let bottom = *contents.last().unwrap();
        let left = Tile::get_left(&contents);
        let right = Tile::get_right(&contents);

        let mut possible_edges: HashSet<u16> = HashSet::new();
        for edge in &[top, bottom, left, right] {
            possible_edges.insert(*edge);
            possible_edges.insert(reverse_10_bits(*edge));
        }

        assert!(possible_edges.len() == 8); // A bit of a hack - ensures there's a unique way to rotate & match an edge

        Tile {
            id,
            contents,
            possible_edges,
        }
    }

    fn line_to_u16(s: &str) -> u16 {
        u16::from_str_radix(&s.replace(".", "0").replace("#", "1"), 2).unwrap()
    }

    fn rotate(&mut self) {
        // Anti-clockwise
        let mut new_contents: Vec<u16> = Vec::new();

        for shift in 0..TILE_WIDTH {
            new_contents.push(Tile::get_right(
                &self
                    .contents
                    .iter()
                    .map(|&u| u >> shift)
                    .collect::<Vec<u16>>(),
            ));
        }

        self.contents = new_contents;
    }

    fn flip(&mut self) {
        // About the horizontal axis
        self.contents.reverse();
    }

    fn turn_to_top(&mut self, val: u16) {
        // Flip/ rotate until the top is the value we want
        for _turn in 0..4 {
            if self.top() == val {
                return;
            }

            self.rotate();
        }

        self.flip();

        for _turn in 0..4 {
            if self.top() == val {
                return;
            }

            self.rotate();
        }

        panic!(
            "Couldn't match val {:} on tile {:} [{:}, {:}, {:}, {:}]",
            val,
            self.id,
            self.top(),
            self.left(),
            self.bottom(),
            self.right()
        );
    }

    fn turn_to_left(&mut self, val: u16) {
        self.turn_to_top(reverse_10_bits(val)); // Top to side is flipped on rotate, because of how I've defined the reading direction
        self.rotate();
    }

    fn top(&self) -> u16 {
        self.contents[0]
    }

    fn bottom(&self) -> u16 {
        *self.contents.last().unwrap()
    }

    fn left(&self) -> u16 {
        Tile::get_left(&self.contents)
    }

    fn right(&self) -> u16 {
        Tile::get_right(&self.contents)
    }

    fn get_left(contents: &[u16]) -> u16 {
        Tile::get_edge(contents, true)
    }

    fn get_right(contents: &[u16]) -> u16 {
        Tile::get_edge(contents, false)
    }

    fn get_edge(contents: &[u16], left_not_right: bool) -> u16 {
        let shift = match left_not_right {
            true => TILE_WIDTH - 1,
            false => 0,
        };
        contents.iter().enumerate().fold(0, |acc, (i, b)| {
            acc | (((b & (1 << shift)) >> shift) << (TILE_WIDTH - 1 - i))
        })
    }
}

impl std::hash::Hash for Tile {
    fn hash<H>(&self, state: &mut H)
    where
        H: std::hash::Hasher,
    {
        state.write_u64(self.id);
        state.finish();
    }
}

impl PartialEq for Tile {
    fn eq(&self, other: &Tile) -> bool {
        self.id == other.id
    }
}

impl fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.id)
    }
}

lazy_static! {
    static ref REVERSE_10_BIT_CACHE: Mutex<HashMap<u16, u16>> = Mutex::new(HashMap::new());
}

fn reverse_10_bits(x: u16) -> u16 {
    // assert!(TILE_WIDTH == 10);
    let mut cache = REVERSE_10_BIT_CACHE.lock().unwrap();
    match cache.get(&x) {
        Some(rx) => *rx,
        None => {
            // Lazy implementation for 10-bit numbers only
            let rx = ((x & 0b0_000_000_001) << 9)
                | ((x & 0b0_000_000_010) << 7)
                | ((x & 0b0_000_000_100) << 5)
                | ((x & 0b0_000_001_000) << 3)
                | ((x & 0b0_000_010_000) << 1)
                | ((x & 0b1_000_000_000) >> 9)
                | ((x & 0b0_100_000_000) >> 7)
                | ((x & 0b0_010_000_000) >> 5)
                | ((x & 0b0_001_000_000) >> 3)
                | ((x & 0b0_000_100_000) >> 1);
            cache.insert(x, rx);
            cache.insert(rx, x);

            rx
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reverse() {
        assert!(reverse_10_bits(0b1_000_000_000) == 1);
        assert!(reverse_10_bits(0b1_000_000_000) == 1); // Cached
        assert!(reverse_10_bits(0b0_101_100_110) == 0b011_001_101_0);
    }

    #[test]
    fn test_transform() {
        let mut tile = Tile::from(
            "Tile 2311:
..##.#..#.
##..#.....
#...##..#.
####.#...#
##.##.###.
##...#.###
.#.#.#..##
..#....#..
###...#.#.
..###..###",
        );
        assert!(tile.top() == 0b00110_10010);
        assert!(tile.left() == 0b01111_10010);
        assert!(tile.bottom() == 0b00111_00111);
        assert!(tile.right() == 0b00010_11001);

        tile.rotate();
        assert!(tile.left() == 0b01001_01100);
        assert!(tile.bottom() == 0b01111_10010);
        assert!(tile.right() == 0b11100_11100);
        assert!(tile.top() == 0b00010_11001);

        tile.flip();
        assert!(tile.left() == 0b00110_10010);
        assert!(tile.bottom() == 0b00010_11001);
        assert!(tile.right() == 0b00111_00111);
        assert!(tile.top() == 0b01111_10010);

        tile.turn_to_top(0b00110_10010); // Starting position
        assert!(tile.left() == 0b01111_10010);

        tile.turn_to_left(0b01001_01100);
        assert!(tile.bottom() == 0b01111_10010);

        tile.turn_to_left(0b11100_11100); // A flipped position
        assert!(tile.top() == 0b10011_01000);
    }

    #[test]
    fn test_one() {
        assert!(part_one("data/20_example.txt") == 20899048083289);
    }

    #[test]
    fn test_two() {
        assert!(part_two("data/20_example.txt") == 273);
    }
}
