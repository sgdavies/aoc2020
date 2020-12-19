use crate::file_to_vec;
use std::collections::HashMap;
use regex::Regex;
use lazy_static::lazy_static;

pub(crate) fn part_one(rules: &str, inputs: &str) -> usize {
    let (partial_rules, mut finished_rules) = build_rules(rules);

    let pattern: &str = &format!("^{}$", rule_for_val(0, &partial_rules, &mut finished_rules));
    solve(inputs, pattern)
}

pub(crate) fn part_two(rules: &str, inputs: &str) -> usize {
    let (mut partial_rules, mut finished_rules) = build_rules(rules);

    partial_rules.insert(8, "42 | 42 8".to_string());
    partial_rules.insert(11, "42 31 | 42 11 31".to_string());

    // Current approach infinite-loops on rules 8 & 11 which refer to themselves
    unimplimented!();
    // let pattern: &str = &format!("^{}$", rule_for_val(0, &partial_rules, &mut finished_rules));
    // solve(inputs, pattern)
}

fn build_rules(rules: &str) -> (HashMap<u32, String>, HashMap<u32, String>) {
    let mut partial_rules: HashMap<u32, String> = HashMap::new();
    let mut finished_rules: HashMap<u32, String> = HashMap::new();

    for rule in file_to_vec(rules).iter() {
        let mut parts = rule.split(": ");
        let number = parts.next().unwrap().parse::<u32>().unwrap();
        let rule = parts.next().unwrap();

        match rule {
            "\"a\"" => finished_rules.insert(number, "a".to_string()),
            "\"b\"" => finished_rules.insert(number, "b".to_string()),
            rule => partial_rules.insert(number, rule.to_string()),
        };
    }

    (partial_rules, finished_rules)
}

fn solve(inputs: &str, pattern: &str) -> usize {
    let re = Regex::new(pattern).unwrap();
    file_to_vec(inputs).iter().filter(|s| re.is_match(s)).count()
}

lazy_static! {
    static ref HAS_NUMBERS: Regex = Regex::new(r"[0-9]+").unwrap();
}

fn rule_for_val(number: u32, partials: &HashMap<u32, String>, finished: &mut HashMap<u32, String>) -> String {
    match finished.get(&number) {
        Some(rule) => rule.to_string(),
        None => {
            let mut rule = "(".to_string();
            for elem in partials.get(&number).unwrap().split(' ') {
                match elem {
                    "|" => rule.push_str("|"),
                    num => {
                        let num = num.parse::<u32>().unwrap();
                        let sub_rule = rule_for_val(num, partials, finished);

                        if !HAS_NUMBERS.is_match(&sub_rule) {
                            finished.insert(num, sub_rule.to_owned());
                        }

                        rule.push_str(&sub_rule);
                    },
                };
            }
            rule.push_str(")");

            rule
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        // didn't bother with test inputs... 😊
        assert!(part_one("data/19rules.txt", "data/19inputs.txt") == 182);
    }
}