use crate::file_to_vec;
use std::collections::{BTreeMap, HashSet};

pub(crate) fn part_one(filename: &str) -> usize {
    let rows = parse_input(filename);
    let allergens_to_poss_ingrs = get_possible_allergens(&rows);
    let may_contain_allergens = get_maybes(&allergens_to_poss_ingrs);

    rows.iter().fold(0, |acc, row| {
        acc + row.0.difference(&may_contain_allergens).count()
    })
}

fn parse_input(filename: &str) -> Vec<(HashSet<String>, HashSet<String>)> {
    let mut rows: Vec<_> = Vec::new();
    for row in file_to_vec(filename).iter() {
        let mut parts = row.split(" (contains ");
        let ingredients: HashSet<String> = parts
            .next()
            .unwrap()
            .split(' ')
            .map(|s| s.to_string())
            .collect();
        let allergen_list = parts.next().unwrap();
        let allergens: HashSet<String> = allergen_list[0..allergen_list.len() - 1]
            .split(", ")
            .map(|s| s.to_string())
            .collect();
        rows.push((ingredients, allergens));
    }

    rows
}

fn get_possible_allergens(
    rows: &[(HashSet<String>, HashSet<String>)],
) -> BTreeMap<String, HashSet<String>> {
    let mut allergens_to_poss_ingrs: BTreeMap<String, HashSet<String>> = BTreeMap::new();
    for row in rows.iter() {
        let ingredients = &row.0;
        let allergens = &row.1;
        for allergen in allergens {
            allergens_to_poss_ingrs.insert(
                allergen.clone(),
                match allergens_to_poss_ingrs.get(allergen) {
                    Some(other_ingredients) => other_ingredients
                        .intersection(ingredients)
                        .map(|s| s.to_string())
                        .collect(),
                    None => ingredients.clone(),
                },
            );
        }
    }

    allergens_to_poss_ingrs
}

fn get_maybes(allergens_to_poss_ingrs: &BTreeMap<String, HashSet<String>>) -> HashSet<String> {
    allergens_to_poss_ingrs
        .values()
        .fold(HashSet::new(), |acc, v| {
            acc.union(v).map(|s| s.to_string()).collect()
        })
}

pub(crate) fn part_two(filename: &str) -> String {
    let rows = parse_input(filename);
    let mut allergens_to_poss_ingrs = get_possible_allergens(&rows);

    // println!("{:?}", allergens_to_poss_ingrs);

    // Lazy - lifted from similar problem on day 16 (though that was HashSet, not Map)
    loop {
        if allergens_to_poss_ingrs.values().all(|c| c.len() == 1) {
            break;
        }

        let uniques: HashSet<String> = allergens_to_poss_ingrs
            .iter()
            .filter(|(_k, v)| v.len() == 1)
            .map(|(_k, v)| v.iter().next().unwrap().to_owned())
            .collect();

        // For sets that are not solved (len!=1), remove the solved ones
        allergens_to_poss_ingrs = allergens_to_poss_ingrs
            .into_iter()
            .map(|(k, v)| match v.len() {
                1 => (k, v),
                _ => (k, v.difference(&uniques).map(|s| s.to_string()).collect()),
            })
            .collect();
    }

    // println!("{:?}", allergens_to_poss_ingrs);

    allergens_to_poss_ingrs
        .values()
        .map(|v| v.iter().next().unwrap().to_string())
        .collect::<Vec<String>>()
        .join(",")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_one() {
        assert!(part_one("data/21_example.txt") == 5);
    }

    #[test]
    fn test_two() {
        assert!(part_two("data/21_example.txt") == "mxmxvkd,sqjhc,fvjkl".to_owned());
    }
}
