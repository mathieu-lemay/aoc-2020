use std::fmt::Display;
use std::time::Instant;

use regex::Regex;

use aoc_2020::get_input;

fn solve(input: &[String]) -> (impl Display, impl Display) {
    let re = Regex::new(r"(\d+)-(\d+) ([a-z]): ([a-z]+)").unwrap();

    let mut nbvalid1 = 0;
    let mut nbvalid2 = 0;

    for l in input {
        let cap = re.captures(&l).unwrap();
        let min: usize = cap.get(1).unwrap().as_str().parse().unwrap();
        let max: usize = cap.get(2).unwrap().as_str().parse().unwrap();
        let letter = cap.get(3).unwrap().as_str().chars().next().unwrap();
        let pass = cap.get(4).unwrap().as_str();
        let count = pass.chars().filter(|c| c == &letter).count();
        if count >= min && count <= max {
            nbvalid1 += 1;
        }

        let a = if pass.chars().nth(min - 1).unwrap() == letter {
            1
        } else {
            0
        };
        let b = if pass.chars().nth(max - 1).unwrap() == letter {
            1
        } else {
            0
        };
        if a + b == 1 {
            nbvalid2 += 1;
        }
    }

    (nbvalid1, nbvalid2)
}

fn main() {
    let input = get_input("d02.txt");

    let start = Instant::now();

    let (r1, r2) = solve(input.as_slice());

    let t = start.elapsed().as_micros() as f64 / 1000.0;

    println!("Part 1: {}", r1);
    println!("Part 2: {}", r2);
    println!("Duration: {:.3}ms", t);
}
