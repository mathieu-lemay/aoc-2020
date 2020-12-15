use std::collections::HashMap;
use std::fmt::Display;
use std::time::Instant;

use aoc_2020::get_input;

fn get_number_at_turn(input: &Vec<u64>, turn: usize) -> usize {
    let mut numbers = HashMap::new();
    let mut current_turn = input.len();

    for (t, &n) in input.iter().enumerate() {
        current_turn = t + 1;
        numbers.insert(n as usize, (current_turn, current_turn));
    }

    let mut current_number = input[current_turn - 1] as usize;

    while current_turn < turn {
        current_turn += 1;

        current_number = match numbers.get(&current_number) {
            Some((t1, t2)) => t2 - t1,
            None => 0,
        };

        let last_turn = match numbers.get(&current_number) {
            Some((_, t)) => *t,
            None => current_turn,
        };

        numbers.insert(current_number, (last_turn, current_turn));
    }

    current_number
}

fn solve(input: &str) -> (impl Display, impl Display) {
    let input = input
        .split(',')
        .map(|s| s.parse().unwrap())
        .collect::<Vec<u64>>();

    let p1 = get_number_at_turn(&input, 2020);
    let p2 = get_number_at_turn(&input, 30000000);

    (p1, p2)
}

fn main() {
    let input = get_input("d15.txt");

    let start = Instant::now();

    let (r1, r2) = solve(&input[0]);

    let t = start.elapsed().as_micros() as f64 / 1000.0;

    println!("Part 1: {}", r1);
    println!("Part 2: {}", r2);
    println!("Duration: {:.3}ms", t);
}

#[cfg(test)]
mod tests {
    use crate::get_number_at_turn;

    #[test]
    fn test_get_number_at_turn() {
        assert_eq!(4, get_number_at_turn(&vec![0, 3, 6], 9));
        assert_eq!(0, get_number_at_turn(&vec![0, 3, 6], 10));
        assert_eq!(436, get_number_at_turn(&vec![0, 3, 6], 2020));
        assert_eq!(1, get_number_at_turn(&vec![1, 3, 2], 2020));
        assert_eq!(10, get_number_at_turn(&vec![2, 1, 3], 2020));
        assert_eq!(27, get_number_at_turn(&vec![1, 2, 3], 2020));
        assert_eq!(78, get_number_at_turn(&vec![2, 3, 1], 2020));
        assert_eq!(438, get_number_at_turn(&vec![3, 2, 1], 2020));
        assert_eq!(1836, get_number_at_turn(&vec![3, 1, 2], 2020));
    }

    #[test]
    fn test_part1() {
        assert_eq!(203, get_number_at_turn(&vec![0, 5, 4, 1, 10, 14, 7], 2020));
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            9007186,
            get_number_at_turn(&vec![0, 5, 4, 1, 10, 14, 7], 30_000_000)
        );
    }
}
