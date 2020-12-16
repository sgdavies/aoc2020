use crate::file_to_vec;
use std::collections::HashSet;

pub(crate) fn solve_8a(filename: &str) -> isize {
    let instructions = get_instructions(filename);

    match run_program(&instructions) {
        Ok(acc) => panic!("Program terminated! with value {:?}", acc),
        Err(acc) => acc,
    }
}

pub(crate) fn solve_8b(filename: &str) -> isize {
    let initial_instructions = get_instructions(filename);

    let nop_or_jmp: Vec<usize> = initial_instructions
        .iter()
        .enumerate()
        .filter(|(_i, v)| v.is_nop_or_jmp())
        .map(|(i, _)| i)
        .collect();
    for index in nop_or_jmp.iter() {
        if let Ok(answer) = run_program(&swap_nop_jmp(&initial_instructions, *index)) {
            return answer;
        }
    }
    panic!("Couldn't find any versions which terminated");
}

fn get_instructions(filename: &str) -> Vec<Opcode> {
    file_to_vec(filename)
        .iter()
        .map(|s| Opcode::from(s))
        .collect()
}

fn swap_nop_jmp(instructions: &[Opcode], index: usize) -> Vec<Opcode> {
    let mut new_instructions = instructions.to_owned();
    match new_instructions[index] {
        Opcode::Nop(val) => {
            new_instructions[index] = Opcode::Jmp(val);
        }
        Opcode::Jmp(val) => {
            new_instructions[index] = Opcode::Nop(val);
        }
        Opcode::Acc(_) => panic!("Can't toggle Acc instruction (at index {:})", index),
    }
    new_instructions
}

// Ok = program terminated; here is the acc value.
// Err = program looped, here is the acc value just before entering the infinite loop.
fn run_program(instructions: &[Opcode]) -> Result<isize, isize> {
    let mut visited: HashSet<isize> = HashSet::new();

    let mut ip = 0isize;
    let mut acc = 0isize;

    loop {
        if !visited.insert(ip) {
            return Err(acc);
        };

        if let Some(op) = &instructions.get(ip as usize) {
            match op {
                Opcode::Nop(_) => {
                    ip += 1;
                }
                Opcode::Acc(val) => {
                    acc += val;
                    ip += 1;
                }
                Opcode::Jmp(val) => {
                    ip += val;
                }
            }
        } else {
            return Ok(acc);
        }
    }
}

#[derive(Clone)]
enum Opcode {
    Nop(isize),
    Acc(isize),
    Jmp(isize),
}

impl Opcode {
    fn from(s: &str) -> Opcode {
        let mut words = s.split_whitespace();
        let opcode = words.next().unwrap();
        let value = words.next().and_then(|w| w.parse::<isize>().ok()).unwrap();
        match opcode {
            "nop" => Opcode::Nop(value),
            "acc" => Opcode::Acc(value),
            "jmp" => Opcode::Jmp(value),
            _ => panic!("Can't parse opcode {}", opcode),
        }
    }

    fn is_nop_or_jmp(&self) -> bool {
        match *self {
            Opcode::Nop(_) => true,
            Opcode::Jmp(_) => true,
            _ => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert!(solve_8a("data/8_example.txt") == 5);
        assert!(solve_8b("data/8_example.txt") == 8);
    }
}
