use std::fmt::Display;
use std::time::Instant;

use aoc_2020::get_input;

fn solve(input: &[String]) -> (impl Display, impl Display) {
    (0, 0)
}

fn main() {
    let input = get_input("d06.txt");

    let start = Instant::now();

    let (r1, r2) = solve(input.as_slice());

    let t = start.elapsed().as_micros() as f64 / 1000.0;

    println!("Part 1: {}", r1);
    println!("Part 2: {}", r2);
    println!("Duration: {:.3}ms", t);
}
