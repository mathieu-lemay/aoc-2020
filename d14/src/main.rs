#[macro_use]
extern crate lazy_static;

use std::fmt::Display;
use std::time::Instant;

use regex::Regex;

use aoc_2020::get_input;
use std::collections::HashMap;

lazy_static! {
    static ref MASK_REGEX: Regex = Regex::new(r"mask = ([01X]+)").unwrap();
    static ref MEMORY_REGEX: Regex = Regex::new(r"mem\[(\d+)\] = (\d+)").unwrap();
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
struct Mask {
    value: u64,
    active_bits: u64,
}

enum Operation {
    Mask(Mask),
    Memory(u64, u64),
}

fn parse_mask(mask_str: &str) -> Mask {
    let mut value: u64 = 0;
    let mut active_bits: u64 = 0;

    for c in mask_str.chars() {
        value <<= 1;
        active_bits <<= 1;

        match c {
            '0' => {
                active_bits |= 1;
            }
            '1' => {
                value |= 1;
                active_bits |= 1;
            }
            _ => {}
        }
    }

    Mask { value, active_bits }
}

fn apply_mask_xor(value: u64, mask: &Mask) -> u64 {
    let real_mask = (value ^ mask.value) & mask.active_bits;

    value ^ real_mask
}

fn apply_mask_or(value: u64, mask: &Mask) -> u64 {
    value | mask.value
}

fn flip_mask(m: u64) -> u64 {
    !m & ((1 << 36) - 1)
}

fn get_idx_active_bits(v: u64) -> Vec<usize> {
    let mut indexes = Vec::new();
    let mut idx = 0;
    let mut v = v;

    while v > 0 {
        if v & 1 == 1 {
            indexes.push(idx);
        }

        idx += 1;
        v >>= 1;
    }

    indexes
}

fn get_addresses(addr: u64, mask: &Mask) -> Vec<u64> {
    let mut addresses = Vec::new();

    let addr = apply_mask_or(addr, mask);
    let indexes = get_idx_active_bits(flip_mask(mask.active_bits));

    for i in 0..2u32.pow(indexes.len() as u32) {
        let mut mask = 0u64;
        let mut v = i as u64;

        for idx in indexes.iter() {
            mask |= ((v & 1) << idx) as u64;

            v >>= 1;
        }

        addresses.push(addr ^ mask);
    }

    addresses
}

fn parse_op(op_str: &String) -> Operation {
    if let Some(cap) = MASK_REGEX.captures(op_str) {
        Operation::Mask(parse_mask(cap.get(1).unwrap().as_str()))
    } else if let Some(cap) = MEMORY_REGEX.captures(op_str) {
        Operation::Memory(
            cap.get(1).unwrap().as_str().parse().unwrap(),
            cap.get(2).unwrap().as_str().parse().unwrap(),
        )
    } else {
        panic!("Unable to parse '{}'", op_str);
    }
}

fn parse(input: &[String]) -> Vec<Operation> {
    input.iter().map(parse_op).collect::<Vec<Operation>>()
}

fn part1(ops: &Vec<Operation>) -> u64 {
    let mut mem: HashMap<u64, u64> = HashMap::new();
    let mut mask = Mask {
        value: 0,
        active_bits: 0,
    };

    for op in ops {
        match op {
            Operation::Mask(m) => mask = *m,
            Operation::Memory(addr, val) => {
                mem.insert(*addr, apply_mask_xor(*val, &mask));
            }
        };
    }
    mem.values().sum()
}

fn part2(ops: &Vec<Operation>) -> u64 {
    let mut mem: HashMap<u64, u64> = HashMap::new();
    let mut mask = Mask {
        value: 0,
        active_bits: 0,
    };

    for op in ops {
        match op {
            Operation::Mask(m) => mask = *m,
            Operation::Memory(addr, val) => {
                for addr in get_addresses(*addr, &mask) {
                    mem.insert(addr, *val);
                }
            }
        };
    }

    mem.values().sum()
}

fn solve(input: &[String]) -> (impl Display, impl Display) {
    let ops = parse(input);
    let p1 = part1(&ops);
    let p2 = part2(&ops);

    (p1, p2)
}

fn main() {
    let input = get_input("d14.txt");

    let start = Instant::now();

    let (r1, r2) = solve(input.as_slice());

    let t = start.elapsed().as_micros() as f64 / 1000.0;

    println!("Part 1: {}", r1);
    println!("Part 2: {}", r2);
    println!("Duration: {:.3}ms", t);
}

#[cfg(test)]
mod tests {
    use crate::{apply_mask_xor, get_addresses, parse, parse_mask, part1, part2, Mask};
    use aoc_2020::get_input;

    #[test]
    fn test_parse_mask() {
        let expected = Mask {
            value: 0b000000000000000000000000000001000000,
            active_bits: 0b000000000000000000000000000001000010,
        };

        assert_eq!(expected, parse_mask("XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X"));
    }

    #[test]
    fn test_apply_mask() {
        let mask = parse_mask("XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X");
        assert_eq!(73, apply_mask_xor(11, &mask));
        assert_eq!(101, apply_mask_xor(101, &mask));
        assert_eq!(64, apply_mask_xor(64, &mask));
    }

    #[test]
    fn test_get_addresses() {
        let mask = parse_mask("000000000000000000000000000000X1001X");
        let mut addresses = get_addresses(42, &mask);
        addresses.sort();
        assert_eq!(vec![26, 27, 58, 59], addresses);

        let mask = parse_mask("00000000000000000000000000000000X0XX");
        let mut addresses = get_addresses(26, &mask);
        addresses.sort();
        assert_eq!(vec![16, 17, 18, 19, 24, 25, 26, 27], addresses);
    }

    #[test]
    fn test_part1() {
        let input = "mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
mem[8] = 11
mem[7] = 101
mem[8] = 0";

        let input = input.split("\n").map(String::from).collect::<Vec<String>>();
        let ops = parse(&input);

        assert_eq!(165, part1(&ops));
    }

    #[test]
    fn test_part2() {
        let input = "mask = 000000000000000000000000000000X1001X
mem[42] = 100
mask = 00000000000000000000000000000000X0XX
mem[26] = 1";

        let input = input.split("\n").map(String::from).collect::<Vec<String>>();
        let ops = parse(&input);

        assert_eq!(208, part2(&ops));
    }
}
