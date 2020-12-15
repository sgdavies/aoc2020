use crate::file_to_vec;
use std::collections::HashMap;

pub(crate) fn part_one(filename: &str) -> u64 {
    let mut memory: HashMap<u32, u64> = HashMap::new();

    let mut or_mask: u64 = 0;
    let mut and_mask: u64 = 0;

    for line in file_to_vec(filename).iter() {
        let mut parts = line.split(" = ");
        let first = parts.next().unwrap();
        let second = parts.next().unwrap();
        match first {
            "mask" => {
                or_mask = u64::from_str_radix(&second.replace("X", "0"), 2).unwrap();
                and_mask = u64::from_str_radix(&second.replace("X", "1"), 2).unwrap();
            }
            mem => {
                // e.g. "mem[3723]"
                if let Ok(address) = mem[4..mem.len() - 1].parse::<u32>() {
                    let value = (second.parse::<u64>().unwrap() | or_mask) & and_mask;
                    memory.insert(address, value);
                }
            }
        }
    }

    memory.values().sum()
}

pub(crate) fn part_two(filename: &str) -> u64 {
    let mut memory: HashMap<u64, u64> = HashMap::new();

    let mut mask: &str = "";

    for line in file_to_vec(filename).iter() {
        let mut parts = line.split(" = ");
        let first = parts.next().unwrap();
        let second = parts.next().unwrap();
        match first {
            "mask" => {
                mask = second;
            }
            mem => {
                if let Ok(address) = mem[4..mem.len() - 1].parse::<u64>() {
                    let value = second.parse::<u64>().unwrap();
                    for addr in addresses_for_mask(mask, address).iter() {
                        memory.insert(*addr, value);
                    }
                } else {
                    panic!("Couldn't parse line: {:?}", line);
                }
            }
        }
    }

    memory.values().sum()
}

fn addresses_for_mask(mask: &str, address: u64) -> Vec<u64> {
    // 0010_10XX0110_X0X01000_1001X010_11XX010X
    static CHAR_0: u8 = 0x30;
    static CHAR_1: u8 = 0x31;
    static CHAR_X: u8 = 0x58;

    assert!(mask.len() == 36);
    let address_bin = format!("{:#038b}", address)[2..].as_bytes().to_owned(); // ignore the '0b' prefix -- which takes up 2 of the #38 chars!!!
    let mask_bin = mask.as_bytes();
    let mut base_address = String::new();

    for i in 0..mask_bin.len() {
        base_address.push(match mask_bin[i] {
            x if x == CHAR_X => 'X',
            x if x == CHAR_1 => '1',
            x if x == CHAR_0 => match address_bin[i] {
                x if x == CHAR_1 => '1',
                x if x == CHAR_0 => '0',
                bad => panic!("Unexpected char in address at index {:}: {:?}", i, bad),
            },
            bad => panic!("Unexpected char in mask at index {:}: {:?}", i, bad),
        });
    }

    let mut finished_addresses: Vec<u64> = Vec::new();
    let mut potential_addresses: Vec<String> = Vec::new();
    potential_addresses.push(base_address);

    while let Some(x_address) = potential_addresses.pop() {
        if let Some(index) = x_address.find('X') {
            potential_addresses.push(replace_x_at(&x_address, index, "0"));
            potential_addresses.push(replace_x_at(&x_address, index, "1"));
        } else {
            finished_addresses.push(u64::from_str_radix(&x_address, 2).unwrap());
        }
    }

    finished_addresses
}

fn replace_x_at(binary_string: &str, index: usize, value: &str) -> String {
    let mut new_string = binary_string[..index].to_owned();
    new_string.push_str(value);
    new_string.push_str(&binary_string[index + 1..]);
    new_string
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_one() {
        assert!(part_one("data/14_example.txt") == 165);
    }

    #[test]
    fn replace() {
        assert!(replace_x_at("0X001XX", 1, "0") == "00001XX");
        assert!(replace_x_at("X1101X", 0, "0") == "01101X");
        assert!(replace_x_at("X1101X", 0, "1") == "11101X");
        assert!(replace_x_at("X1101X", 5, "0") == "X11010");
        assert!(replace_x_at("X1101X", 5, "1") == "X11011");
    }

    #[test]
    fn addresses_from() {
        let addresses = addresses_for_mask("000000000000000000000000000000X1001X", 42);
        println!("{:?}", addresses);
        println!(
            "{:?}",
            addresses
                .iter()
                .map(|&x| format!("{:#b}", x))
                .collect::<Vec<String>>()
        );
        assert!(addresses.iter().any(|&x| x == 26));
        assert!(addresses.iter().any(|&x| x == 27));
        assert!(addresses.iter().any(|&x| x == 58));
        assert!(addresses.iter().any(|&x| x == 59));
    }

    #[test]
    fn test_two() {
        assert!(part_two("data/14_example2.txt") == 208);
    }
}
