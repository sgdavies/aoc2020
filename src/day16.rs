use std::collections::HashSet;
use std::fs;
use std::str::FromStr;

pub(crate) fn part_one(filename: &str) -> u32 {
    let contents =
        fs::read_to_string(filename).unwrap_or_else(|_| panic!("Couldn't read file {}", filename));
    let chunks: Vec<&str> = contents.split("\n\n").collect();

    // First, the constraints
    let mut constraints: Vec<Constraint> = Vec::new();
    for line in chunks[0].split('\n') {
        constraints.push(line.parse().unwrap());
    }

    // Ignore my own ticket for now

    // Last, the list of tickets
    chunks[2].split('\n').fold(0, |acc, line| match line {
        header if header == "nearby tickets:" => 0,
        line => {
            acc + line.split(',').fold(0, |acc2, num| {
                acc2 + {
                    let num = num.parse::<u32>().unwrap();
                    match constraints.iter().any(|c| c.could_be(num)) {
                        true => 0,
                        false => num,
                    }
                }
            })
        }
    })
}

pub(crate) fn part_two(filename: &str, prefix: &str) -> u64 {
    let content =
        fs::read_to_string(filename).unwrap_or_else(|_| panic!("Couldn't read file {}", filename));
    let mut lines = content.lines();

    // First, the constraints
    let mut constraints: HashSet<Constraint> = HashSet::new();
    while let Some(line) = lines.next() {
        if line == "" {
            break;
        }
        constraints.insert(line.parse().unwrap());
    }

    // My ticket
    let _my_header = lines.next();
    let my_ticket: Vec<u32> = lines
        .next()
        .unwrap()
        .split(',')
        .map(|s| s.parse::<u32>().unwrap())
        .collect();

    let _blank = lines.next();
    let _tickets_header = lines.next();

    // The list of tickets
    let mut tickets: Vec<Vec<u32>> = Vec::new();
    for line in lines {
        tickets.push(line.split(',').map(|s| s.parse::<u32>().unwrap()).collect());
    }

    tickets.push(my_ticket);

    // Now start winnowing down which constraint can apply to which field
    let mut values_at_index: Vec<HashSet<u32>> = Vec::new();
    for _ in tickets[0].iter() {
        values_at_index.push(HashSet::new());
    }

    for ticket in tickets.iter() {
        if ticket
            .iter()
            .any(|&val| !constraints.iter().any(|c| c.could_be(val)))
        {
            // invalid ticket - ignore
            continue;
        }

        for (i, &val) in ticket.iter().enumerate() {
            values_at_index[i].insert(val);
        }
    }

    let mut cons_for_index: Vec<HashSet<&Constraint>> = Vec::new();
    for vals in values_at_index.iter() {
        let mut matching_constraints: HashSet<&Constraint> = HashSet::new();
        for constraint in constraints.iter() {
            if vals.iter().all(|&v| constraint.could_be(v)) {
                matching_constraints.insert(constraint);
            }
        }
        cons_for_index.push(matching_constraints);
    }

    // At this point, some constraints could still be in multiple places:
    // [{A}, {A,B}, {A,B,C}] - we need to filter down again.
    loop {
        // println!("{:?}", cons_for_index.iter().map(|c| c.len()).collect::<Vec<usize>>());

        if cons_for_index.iter().all(|c| c.len() == 1) {
            break;
        }

        let uniques: HashSet<&Constraint> = cons_for_index
            .iter()
            .filter(|c| c.len() == 1)
            .map(|c| *c.iter().next().unwrap())
            .collect();

        // ಠ_ಠ
        let mut new_cons_for_index: Vec<HashSet<&Constraint>> = Vec::new();
        for cons in cons_for_index.iter() {
            let new_cons: HashSet<&Constraint> = if cons.len() > 1 {
                let mut tmp_cons: HashSet<&Constraint> = HashSet::new();
                for item in cons.difference(&uniques) {
                    tmp_cons.insert(*item);
                }
                tmp_cons
            } else {
                let mut tmp_cons: HashSet<&Constraint> = HashSet::new();
                tmp_cons.insert(cons.iter().next().unwrap());
                tmp_cons
            };
            new_cons_for_index.push(new_cons);
        }

        cons_for_index = new_cons_for_index;
    }

    let my_ticket: Vec<u32> = tickets.pop().unwrap();

    my_ticket.iter().enumerate().fold(1, |acc, (i, &val)| {
        acc * match cons_for_index.get(i) {
            Some(constraints) => {
                assert!(constraints.len() == 1);
                if constraints.iter().next().unwrap().name.starts_with(prefix) {
                    val as u64
                } else {
                    1
                }
            }
            _ => 1,
        }
    })
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct Constraint {
    name: String,
    lower_group: (u32, u32),
    upper_group: (u32, u32),
    // possible_positions: HashSet<usize>,
}

impl Constraint {
    fn could_be(&self, x: u32) -> bool {
        (self.lower_group.0 <= x && x <= self.lower_group.1)
            || (self.upper_group.0 <= x && x <= self.upper_group.1)
    }
}

impl FromStr for Constraint {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // e.g. "departure location: 31-201 or 227-951"
        let mut words = s.split(": ");
        let name = words.next().unwrap().to_owned();
        let mut limits = words.next().unwrap().split(" or ");

        let lower: Vec<u32> = limits
            .next()
            .unwrap()
            .split('-')
            .map(|n| n.parse::<u32>().unwrap())
            .collect();
        let upper: Vec<u32> = limits
            .next()
            .unwrap()
            .split('-')
            .map(|n| n.parse::<u32>().unwrap())
            .collect();

        Ok(Constraint {
            name,
            lower_group: (lower[0], lower[1]),
            upper_group: (upper[0], upper[1]),
        })
    }
}

#[derive(Clone, Debug)]
struct ParseError;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_one() {
        assert!(part_one("data/16_example.txt") == 71);
    }

    #[test]
    fn test_two() {
        assert!(part_two("data/16_example2.txt", "seat") == 13);
    }
}
