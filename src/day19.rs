use crate::file_to_vec;
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;

pub(crate) fn part_one(rules: &str, inputs: &str) -> usize {
    let (partial_rules, mut finished_rules) = build_rules(rules);

    let pattern: &str = &format!("^{}$", rule_for_val(0, &partial_rules, &mut finished_rules));
    solve(inputs, pattern)
}

pub(crate) fn part_two(rules: &str, inputs: &str) -> usize {
    let (partial_rules, mut finished_rules) = build_rules(rules);

    let rule_42 = rule_for_val(42, &partial_rules, &mut finished_rules);
    let rule_31 = rule_for_val(31, &partial_rules, &mut finished_rules);

    // 8 = "42 | 42 8" => "42+"
    let mut rule_8 = "((".to_string();
    rule_8.push_str(&rule_42);
    rule_8.push_str(")+)");
    finished_rules.insert(8, rule_8);

    // 11: 42 31 | 42 11 31 => AB|AABB|AAABBB|...
    // Not possible with a regular expression 😲
    // Just build up to a sensible-looking limit.
    let mut rule_11 = "(".to_string();
    rule_11.push_str(&rule_42);
    rule_11.push_str(&rule_31);
    for repeats in 2..5 {
        rule_11.push('|');
        for _ in 0..repeats {
            rule_11.push_str(&rule_42);
        }
        for _ in 0..repeats {
            rule_11.push_str(&rule_31);
        }
    }
    rule_11.push(')');
    finished_rules.insert(11, rule_11);

    let pattern: &str = &format!("^{}$", rule_for_val(0, &partial_rules, &mut finished_rules));
    solve(inputs, pattern)
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
    file_to_vec(inputs)
        .iter()
        .filter(|s| re.is_match(s))
        .count()
}

lazy_static! {
    static ref HAS_NUMBERS: Regex = Regex::new(r"[0-9]+").unwrap();
}

fn rule_for_val(
    number: u32,
    partials: &HashMap<u32, String>,
    finished: &mut HashMap<u32, String>,
) -> String {
    match finished.get(&number) {
        Some(rule) => rule.to_string(),
        None => {
            let mut rule = "(".to_string();
            for elem in partials.get(&number).unwrap().split(' ') {
                match elem {
                    "|" => rule.push('|'),
                    num => {
                        let num = num.parse::<u32>().unwrap();
                        let sub_rule = rule_for_val(num, partials, finished);

                        if !HAS_NUMBERS.is_match(&sub_rule) {
                            finished.insert(num, sub_rule.to_owned());
                        }

                        rule.push_str(&sub_rule);
                    }
                };
            }
            rule.push(')');

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
