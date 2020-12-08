use crate::file_to_vec;
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;
use std::collections::HashSet;

lazy_static! {
    static ref BAAAAGS: Regex = Regex::new(r"^(\d+) (\w+ \w+) bags?\.?$").unwrap();
}

pub(crate) fn solve_7a(filename: &str) -> usize {
    let bags = BagCollection::parse(filename);

    let mut shiny_parents: HashMap<String, bool> = match bags.bags_to_parents.get("shiny gold") {
        None => panic!("Couldn't even find a shiny gold bag :-("),
        Some(shiny) => shiny
            .iter()
            .map(|parent| (parent.to_owned(), false))
            .collect(),
    };

    while let Some(unvisited_parent) = shiny_parents
        .iter()
        .filter(|(_, visited)| !**visited)
        .map(|(parent, _)| parent.to_owned())
        .next()
    {
        *shiny_parents.get_mut(&unvisited_parent).unwrap() = true;
        if let Some(new_parents) = bags.bags_to_parents.get(&unvisited_parent) {
            for new_parent in new_parents {
                if shiny_parents.get(new_parent).is_none() {
                    shiny_parents.insert(new_parent.to_owned(), false);
                }
            }
        }
    }

    shiny_parents.len()
}

pub(crate) fn solve_7b(filename: &str) -> u32 {
    let bags = BagCollection::parse(filename);

    let mut contains: HashMap<&str, u32> = HashMap::new();

    count_them(&mut contains, &bags, "shiny gold")
}

fn count_them<'a>(
    contains: &mut HashMap<&'a str, u32>,
    bags: &'a BagCollection,
    name: &'a str,
) -> u32 {
    match contains.get(name) {
        Some(count) => *count,
        None => {
            let count = bags
                .bags_to_children
                .get(name)
                .unwrap()
                .iter()
                .fold(0, |acc, (child, count)| {
                    acc + count * (1 + count_them(contains, bags, child))
                });
            contains.insert(name, count);
            count
        }
    }
}

struct BagCollection {
    bags_to_children: HashMap<String, Vec<(String, u32)>>,
    bags_to_parents: HashMap<String, HashSet<String>>,
}

impl BagCollection {
    fn parse(filename: &str) -> BagCollection {
        let mut btc: HashMap<String, Vec<(String, u32)>> = HashMap::new();
        let mut btp: HashMap<String, HashSet<String>> = HashMap::new();
        for row in file_to_vec(filename) {
            let (parent, child_list) = BagCollection::parse_row(&row);

            for (child, _) in &child_list {
                btp.entry(child.clone())
                    .or_insert_with(HashSet::new)
                    .insert(parent.clone());
            }

            btc.insert(parent, child_list);
        }

        BagCollection {
            bags_to_children: btc,
            bags_to_parents: btp,
        }
    }

    fn parse_row(row: &str) -> (String, Vec<(String, u32)>) {
        // "dark red bags contain 4 bright chartreuse bags.";
        // "drab beige bags contain 5 bright teal bags, 1 faded cyan bag, 2 muted yellow bags, 1 dim lime bag.";
        // "dotted violet bags contain no other bags.";
        let parts: Vec<&str> = row.split(" bags contain ").collect();
        let parent = parts[0].to_string();
        let children = match parts[1] {
            "no other bags." => Vec::new(),
            children => {
                // r"(\d+) (\w \w) bags?[,.]"
                children
                    .split(", ")
                    .map(|bags| match BAAAAGS.captures(bags) {
                        Some(captures) => {
                            (captures[2].to_string(), captures[1].parse::<u32>().unwrap())
                        }
                        None => panic!("Couldn't parse bag: \"{}\"", bags),
                    })
                    .collect()
            }
        };
        (parent, children)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert!(solve_7a("data/7_example.txt") == 4);
        assert!(solve_7b("data/7_example.txt") == 32);
    }
}
