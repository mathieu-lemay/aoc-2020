use std::collections::HashMap;
use std::fmt::Display;
use std::time::Instant;

use aoc_2020::get_input;

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq)]
enum Seat {
    Floor,
    Empty,
    Taken,
}

fn parse(input: &[String]) -> Vec<Vec<Seat>> {
    input
        .iter()
        .map(|s| {
            s.chars()
                .map(|c| match c {
                    '.' => Seat::Floor,
                    'L' => Seat::Empty,
                    '#' => Seat::Taken,
                    _ => panic!("Invalid char: '{}'", c),
                })
                .collect::<Vec<Seat>>()
        })
        .collect::<Vec<Vec<Seat>>>()
}

fn part1(plan: &mut Vec<Vec<Seat>>) -> usize {
    let ajd_indices = vec![
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];

    let h = plan.len();
    let w = plan[0].len();

    loop {
        let mut changes = HashMap::new();

        for r in 0..h {
            for c in 0..w {
                let s = &plan[r][c];
                if s == &Seat::Floor {
                    continue;
                }

                let adj_occupied = ajd_indices
                    .iter()
                    .map(|idx| {
                        let x = r as i32 - idx.0;
                        let y = c as i32 - idx.1;

                        if x >= 0
                            && y >= 0
                            && x < h as i32
                            && y < w as i32
                            && &plan[x as usize][y as usize] == &Seat::Taken
                        {
                            1
                        } else {
                            0
                        }
                    })
                    .sum::<u32>();

                if s == &Seat::Empty && adj_occupied == 0 {
                    changes.insert((r, c), Seat::Taken);
                } else if s == &Seat::Taken && adj_occupied >= 4 {
                    changes.insert((r, c), Seat::Empty);
                }
            }
        }

        if changes.is_empty() {
            break;
        }

        for ((x, y), s) in changes {
            plan[x][y] = s;
        }
    }

    plan.iter()
        .map(|r| r.iter().filter(|&s| s == &Seat::Taken).count())
        .sum()
}

fn find_seat_in_direction(
    plan: &Vec<Vec<Seat>>,
    current_position: (usize, usize),
    direction: &(i32, i32),
) -> Option<Seat> {
    let h = plan.len() as i32;
    let w = plan[0].len() as i32;

    let mut x = current_position.0 as i32;
    let mut y = current_position.1 as i32;

    loop {
        x += direction.0;
        y += direction.1;

        if x < 0 || y < 0 || x >= h || y >= w {
            return None;
        }

        let s = plan[x as usize][y as usize];
        if s != Seat::Floor {
            return Some(s);
        }
    }
}

fn part2(plan: &mut Vec<Vec<Seat>>) -> usize {
    let direction_vectors = vec![
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];

    let h = plan.len();
    let w = plan[0].len();

    loop {
        let mut changes = HashMap::new();

        for r in 0..h {
            for c in 0..w {
                let s = &plan[r][c];
                if s == &Seat::Floor {
                    continue;
                }

                let visible_occupied = direction_vectors
                    .iter()
                    .map(|dir| match find_seat_in_direction(plan, (r, c), dir) {
                        Some(Seat::Taken) => 1,
                        _ => 0,
                    })
                    .sum::<u32>();

                if s == &Seat::Empty && visible_occupied == 0 {
                    changes.insert((r, c), Seat::Taken);
                } else if s == &Seat::Taken && visible_occupied >= 5 {
                    changes.insert((r, c), Seat::Empty);
                }
            }
        }

        if changes.is_empty() {
            break;
        }

        for ((x, y), s) in changes {
            plan[x][y] = s;
        }
    }

    plan.iter()
        .map(|r| r.iter().filter(|&s| s == &Seat::Taken).count())
        .sum()
}

fn solve(input: &[String]) -> (impl Display, impl Display) {
    let mut plan = parse(input);
    let p1 = part1(&mut plan);

    let mut plan = parse(input);
    let p2 = part2(&mut plan);

    (p1, p2)
}

fn main() {
    let input = get_input("d11.txt");

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
        let input = "L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL";

        let input = input.split("\n").map(String::from).collect::<Vec<String>>();
        let res = part1(&mut parse(&input));

        assert_eq!(37, res);
    }

    #[test]
    fn test_p2() {
        let input = "L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL";

        let input = input.split("\n").map(String::from).collect::<Vec<String>>();
        let res = part2(&mut parse(&input));

        assert_eq!(26, res);
    }
}
