use std::fmt::Display;
use std::time::Instant;

use aoc_2020::get_input;

fn get_seat_id(pass: &String) -> i32 {
    let mut min_row = 0;
    let mut max_row = 127;
    let mut min_col = 0;
    let mut max_col = 7;

    for c in pass.chars() {
        match c {
            'F' => max_row -= (max_row + 1 - min_row) / 2,
            'B' => min_row += (max_row + 1 - min_row) / 2,
            'L' => max_col -= (max_col + 1 - min_col) / 2,
            'R' => min_col += (max_col + 1 - min_col) / 2,
            _ => panic!("Invalid char: {}", c),
        }
    }

    min_row * 8 + min_col
}

fn solve(input: &[String]) -> (impl Display, impl Display) {
    let mut seat_ids: Vec<i32> = input.iter().map(get_seat_id).collect();
    seat_ids.sort();
    let p1 = seat_ids[seat_ids.len() - 1];

    let mut prev = seat_ids[0];
    let mut p2 = 0;
    for s in seat_ids {
        if s - prev == 2 {
            p2 = s - 1;
            break;
        }
        prev = s;
    }

    (p1, p2)
}

fn main() {
    let input = get_input("d05.txt");

    let start = Instant::now();

    let (r1, r2) = solve(input.as_slice());

    let t = start.elapsed().as_micros() as f64 / 1000.0;

    println!("Part 1: {}", r1);
    println!("Part 2: {}", r2);
    println!("Duration: {:.3}ms", t);
}

#[cfg(test)]
mod tests {
    use crate::get_seat_id;

    #[test]
    fn test_get_seat_id() {
        assert_eq!(357, get_seat_id(&String::from("FBFBBFFRLR")))
    }
}
