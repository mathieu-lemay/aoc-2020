use std::collections::VecDeque;
use std::fmt::Display;
use std::time::Instant;

use aoc_2020::get_input_as_int;

fn part1(input: &[i64], sample_size: usize) -> i64 {
    let mut sample: VecDeque<i64> = input.iter().take(sample_size).copied().collect();

    for tgt in input.iter().skip(sample_size) {
        let valid = sample.iter().any(|n| sample.contains(&(tgt - n)));
        if valid {
            sample.pop_front();
            sample.push_back(*tgt);
        } else {
            return *tgt;
        }
    }

    panic!("Invalid number not found");
}

fn part2(input: &[i64], target: i64) -> (i64, i64) {
    let mut s = 0;
    let mut nums = VecDeque::new();

    for n in input.iter().copied() {
        if n >= target {
            break;
        }

        s += n;
        nums.push_back(n);

        while s > target {
            let k = nums.pop_front().unwrap();
            s -= k;
        }
        if s == target {
            return (*nums.iter().min().unwrap(), *nums.iter().max().unwrap());
        }
    }

    panic!("Sequence not found");
}

fn solve(input: &[i64]) -> (impl Display, impl Display) {
    let p1 = part1(input, 25);
    let p2 = part2(input, p1);
    (p1, p2.0 + p2.1)
}

fn main() {
    let input = get_input_as_int("d09.txt");

    let start = Instant::now();

    let (r1, r2) = solve(input.as_slice());

    let t = start.elapsed().as_micros() as f64 / 1000.0;

    println!("Part 1: {}", r1);
    println!("Part 2: {}", r2);
    println!("Duration: {:.3}ms", t);
}

#[cfg(test)]
mod tests {
    use crate::{part1, part2};

    #[test]
    fn test_p1() {
        let input = "35
20
15
25
47
40
62
55
65
95
102
117
150
182
127
219
299
277
309
576";

        let input = input
            .split("\n")
            .map(|s| s.parse().unwrap())
            .collect::<Vec<i64>>();
        let res = part1(&input, 5);
        assert_eq!(127, res);
    }

    #[test]
    fn test_p2() {
        let input = "35
20
15
25
47
40
62
55
65
95
102
117
150
182
127
219
299
277
309
576";

        let input = input
            .split("\n")
            .map(|s| s.parse().unwrap())
            .collect::<Vec<i64>>();
        let res = part2(&input, 127);
        assert_eq!((15, 47), res);
    }
}
