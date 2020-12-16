use std::collections::HashSet;
use std::fs;

pub fn solve_6a(filename: &str) -> usize {
    let contents =
        fs::read_to_string(filename).unwrap_or_else(|_| panic!("Couldn't read file {}", filename));
    let groups: Vec<Group> = contents.split("\n\n").map(Group::new).collect();

    groups.iter().map(|group| group.anyones()).sum()
}

pub fn solve_6b(filename: &str) -> usize {
    let contents =
        fs::read_to_string(filename).unwrap_or_else(|_| panic!("Couldn't read file {}", filename));
    let groups: Vec<Group> = contents.split("\n\n").map(Group::new).collect();

    groups.iter().map(|group| group.everyones()).sum()
}

struct Group<'a> {
    people: Vec<&'a str>,
}

impl Group<'_> {
    fn new(input: &str) -> Group {
        Group {
            people: input.split('\n').collect(),
        }
    }

    fn anyones(&self) -> usize {
        self.people
            .iter()
            .fold(HashSet::new(), |set, person| {
                let mine: HashSet<_> = person.chars().collect();
                set.union(&mine).copied().collect()
            })
            .len()
    }

    fn everyones(&self) -> usize {
        self.people[0]
            .chars()
            .filter(|&c| self.people.iter().all(|person| person.contains(c)))
            .count()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert!(solve_6a("data/6_example.txt") == 11);
        assert!(solve_6b("data/6_example.txt") == 6);
    }
}
