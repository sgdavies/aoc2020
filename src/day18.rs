use crate::file_to_vec;

pub(crate) fn part_one(filename: &str) -> u64 {
    file_to_vec(filename)
        .iter()
        .fold(0, |acc, line| acc + eval(line, false))
}

pub(crate) fn part_two(filename: &str) -> u64 {
    file_to_vec(filename)
        .iter()
        .fold(0, |acc, line| acc + eval(line, true))
}

fn eval(line: &str, with_precedence: bool) -> u64 {
    let line = line.replace(' ', "");

    let mut chain: Vec<(Op, u64)> = Vec::new();
    let mut op = Some(Op::Add); // First op is "+ (first number)"
    let mut i: usize = 0;

    while i < line.len() {
        match op {
            Some(operation) => {
                // We're looking for a value
                let value = if &line[i..i + 1] == "(" {
                    // find closing brace, call eval() on contents, increment i appropriately
                    let mut j = i + 1;
                    let mut paren_count = 1;
                    while paren_count > 0 {
                        match &line[j..j + 1] {
                            "(" => paren_count += 1,
                            ")" => paren_count -= 1,
                            _ => {}
                        }
                        j += 1;
                    }
                    // Get the ends right - we want to pass through the paren contents,
                    // and then start next iter after the closing paren
                    let value = eval(&line[i + 1..j - 1], with_precedence);
                    i = j;
                    value
                } else {
                    // it's a number - collect the relevant chars
                    let mut j = i;
                    while j < line.len() && &line[j..j + 1] != "+" && &line[j..j + 1] != "*" {
                        j += 1;
                    }

                    let value = line[i..j].parse::<u64>().unwrap();
                    i = j;
                    value
                };

                chain.push((operation, value));
                op = None;
            }
            None => {
                op = match &line[i..i + 1] {
                    "+" => Some(Op::Add),
                    "*" => Some(Op::Multiply),
                    unexpected => panic!("Unexpected at {:}: {:}", i, unexpected),
                };
                i += 1;
            }
        }
    }

    match with_precedence {
        true => {
            // collect into chains of addition, split by multiplies
            let mut chainz: Vec<Vec<u64>> = Vec::new();
            let mut addz: Vec<u64> = Vec::new();
            for (op, value) in chain.iter() {
                match op {
                    Op::Add => addz.push(*value),
                    Op::Multiply => {
                        chainz.push(addz);
                        addz = Vec::new();
                        addz.push(*value);
                    }
                }
            }
            chainz.push(addz);

            chainz
                .iter()
                .fold(1, |macc, adds| macc * adds.iter().sum::<u64>())
        }
        false => chain.iter().fold(0, |acc, (op, val)| op.combine(acc, *val)),
    }
}

enum Op {
    Add,
    Multiply,
}

impl Op {
    fn combine(&self, a: u64, b: u64) -> u64 {
        match self {
            Op::Add => a + b,
            Op::Multiply => a * b,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parens() {
        assert!(eval("5", false) == 5);
        assert!(eval("5 + 2", false) == 7);
        assert!(eval("(5)", false) == 5);
        assert!(eval("((5))", false) == 5);
        assert!(eval("(((5)) + (1 * (2)))", false) == 7);
    }

    #[test]
    fn test_one() {
        assert!(eval("1 + 2 * 3 + 4 * 5 + 6", false) == 71);
        assert!(eval("1 + (2 * 3) + (4 * (5 + 6))", false) == 51);
        assert!(eval("2 * 3 + (4 * 5)", false) == 26);
        assert!(eval("5 + (8 * 3 + 9 + 3 * 4 * 3)", false) == 437);
        assert!(eval("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))", false) == 12240);
        assert!(eval("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2", false) == 13632);
    }

    #[test]
    fn test_two() {
        assert!(eval("1 + 2 * 3 + 4 * 5 + 6", true) == 231);
        assert!(eval("1 + (2 * 3) + (4 * (5 + 6))", true) == 51);
        assert!(eval("2 * 3 + (4 * 5)", true) == 46);
        assert!(eval("5 + (8 * 3 + 9 + 3 * 4 * 3)", true) == 1445);
        assert!(eval("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))", true) == 669060);
        assert!(eval("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2", true) == 23340);
    }
}
