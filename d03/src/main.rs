use std::fmt::Display;
use std::time::Instant;

use aoc_2020::get_input;

fn is_tree(row: &str, row_num: usize, xmult: usize) -> bool {
    row.chars().nth((row_num * xmult) % row.len()).unwrap() == '#'
}

fn solve(input: &[String]) -> (impl Display, impl Display) {
    let number1 = input
        .iter()
        .enumerate()
        .filter(|(idx, row)| is_tree(row, *idx, 3))
        .count();

    let number2: usize = vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)]
        .iter()
        .map(|(x, y)| {
            input
                .iter()
                .step_by(*y)
                .enumerate()
                .filter(|(idx, row)| is_tree(row, *idx, *x))
                .count()
        })
        .product();

    (number1, number2)
}

fn main() {
    let input = get_input("d03.txt");

    let start = Instant::now();

    let (r1, r2) = solve(input.as_slice());

    let t = start.elapsed().as_micros() as f64 / 1000.0;

    println!("Part 1: {}", r1);
    println!("Part 2: {}", r2);
    println!("Duration: {:.3}ms", t);
}
