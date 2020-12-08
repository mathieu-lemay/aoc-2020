use std::fmt::Display;
use std::time::Instant;

use aoc_2020::get_input;
use std::collections::HashSet;

#[derive(Debug)]
enum Instr {
    NOP(i32),
    ACC(i32),
    JMP(i32),
}

fn parse(input: &[String]) -> Vec<Instr> {
    let mut instructions = Vec::with_capacity(input.len());

    for l in input {
        let mut tokens = l.split(" ");

        let op = tokens.next().unwrap();
        let n = tokens.next().unwrap().parse().unwrap();

        let instr = match op {
            "nop" => Instr::NOP(n),
            "acc" => Instr::ACC(n),
            "jmp" => Instr::JMP(n),
            _ => panic!("Unsupported op: {}", op),
        };

        instructions.push(instr);
    }

    instructions
}

fn part1(instructions: &Vec<Instr>) -> i32 {
    let mut acc: i32 = 0;
    let mut pc: i32 = 0;

    let mut visited_ops = HashSet::new();

    loop {
        if visited_ops.contains(&pc) {
            return acc;
        }

        visited_ops.insert(pc);

        let instr = instructions.get(pc as usize).unwrap();

        match *instr {
            Instr::NOP(_) => {
                pc += 1;
            }
            Instr::ACC(n) => {
                acc += n;
                pc += 1;
            }
            Instr::JMP(n) => pc += n,
        }
    }
}

fn exec_with_switch(instructions: &Vec<Instr>, idx_to_switch: usize) -> (bool, i32) {
    let mut acc: i32 = 0;
    let mut pc: i32 = 0;

    println!("Exec with switch: {}", idx_to_switch);

    let mut visited_ops = HashSet::new();
    let nb_ops = instructions.len() as i32;

    while pc != nb_ops {
        if visited_ops.contains(&pc) {
            return (false, acc);
        }

        visited_ops.insert(pc);

        let instr = instructions.get(pc as usize).expect("Invalid index");

        match *instr {
            Instr::NOP(n) => {
                if pc as usize == idx_to_switch {
                    pc += n;
                } else {
                    pc += 1;
                }
            }
            Instr::ACC(n) => {
                acc += n;
                pc += 1;
            }
            Instr::JMP(n) => {
                if pc as usize == idx_to_switch {
                    pc += 1;
                } else {
                    pc += n
                }
            }
        }
    }

    (true, acc)
}

fn part2(instructions: &Vec<Instr>) -> i32 {
    for (idx, instr) in instructions.iter().enumerate() {
        let res = match instr {
            Instr::NOP(_) | Instr::JMP(_) => Some(exec_with_switch(instructions, idx)),
            _ => None,
        };

        if let Some((did_halt, acc)) = res {
            if did_halt {
                return acc;
            }
        }
    }

    panic!("Program never halted");
}

fn solve(input: &[String]) -> (impl Display, impl Display) {
    let instructions = parse(input);

    let p1 = part1(&instructions);
    let p2 = part2(&instructions);

    (p1, p2)
}

fn main() {
    let input = get_input("d08.txt");

    let start = Instant::now();

    let (r1, r2) = solve(input.as_slice());

    let t = start.elapsed().as_micros() as f64 / 1000.0;

    println!("Part 1: {}", r1);
    println!("Part 2: {}", r2);
    println!("Duration: {:.3}ms", t);
}

#[cfg(test)]
mod tests {
    use crate::{parse, part1, part2};

    #[test]
    fn test_p1() {
        let input = "nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6";

        let instr = parse(&input.split("\n").map(String::from).collect::<Vec<String>>());
        let res = part1(&instr);
        assert_eq!(5, res);
    }

    #[test]
    fn test_p2() {
        let input = "nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6";

        let instr = parse(&input.split("\n").map(String::from).collect::<Vec<String>>());
        let res = part2(&instr);
        assert_eq!(8, res);
    }
}
