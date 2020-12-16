use crate::file_to_vec;

pub fn solve_5a() -> u32 {
    *seat_ids().iter().max().unwrap()
}

pub fn solve_5b() -> u32 {
    let ids = seat_ids();
    let min: u32 = *ids.iter().min().unwrap();
    let max: u32 = *ids.iter().max().unwrap();
    let sum_all: u32 = (min..max + 1).sum();
    let sum_missing: u32 = seat_ids().iter().sum();
    sum_all - sum_missing
}

fn seat_ids() -> Vec<u32> {
    file_to_vec("data/5a.txt")
        .iter()
        .map(|s| Seat::new(s).seat_id())
        .collect()
}

struct Seat<'a> {
    row: &'a str,
    column: &'a str,
}

impl Seat<'_> {
    fn new(data: &str) -> Seat {
        Seat {
            row: &data[..7],
            column: &data[7..],
        }
    }

    fn row(&self) -> u32 {
        self.row.chars().enumerate().fold(0, |acc, (i, c)| {
            acc + if c == 'B' { 2u32.pow(6 - i as u32) } else { 0 }
        })
    }

    fn column(&self) -> u32 {
        self.column.chars().enumerate().fold(0, |acc, (i, c)| {
            acc + if c == 'R' { 2u32.pow(2 - i as u32) } else { 0 }
        })
    }

    fn seat_id(&self) -> u32 {
        8 * self.row() + self.column()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert!(Seat::new("FFFFFFFLLL").seat_id() == 0);
        assert!(Seat::new("BFFFBBFRRR").seat_id() == 567);
        assert!(Seat::new("FFFBBBFRRR").seat_id() == 119);
        assert!(Seat::new("BBFFBBFRLL").seat_id() == 820);

        assert!(solve_5a() == 980);
    }
}
