use std::fmt::Display;
use std::time::Instant;

use itertools::Itertools;

use aoc_2020::get_input_as_int;
use std::collections::VecDeque;

fn part1(input: &[i64]) -> i64 {
    let values = input.iter().copied().sorted().collect::<Vec<i64>>();

    let mut d1 = 0;
    let mut d3 = 1;

    let mut last = 0;

    for i in values {
        match i - last {
            1 => d1 += 1,
            3 => d3 += 1,
            _ => {}
        }

        last = i;
    }

    d1 * d3
}

fn part2(input: &[i64]) -> i64 {
    let mut values = input
        .iter()
        .sorted()
        .tuple_windows::<(&i64, &i64)>()
        .map(|(i, j)| j - i)
        .collect::<VecDeque<i64>>();
    values.push_front(1);

    // See https://stackoverflow.com/a/32717990
    let values = values
        .iter()
        .copied()
        .map(|c| (c, 1))
        .coalesce(|(c, n), (d, m)| {
            if c == d {
                Ok((c, n + m))
            } else {
                Err(((c, n), (d, m)))
            }
        })
        .collect::<Vec<(i64, i64)>>();

    values
        .iter()
        .filter(|&&t| t.0 == 1)
        .map(|&t| match t.1 {
            1 => 1,
            2 => 2,
            3 => 4,
            4 => 7,
            _ => panic!("Invalid group length"),
        })
        .product::<i64>()
}

fn solve(input: &[i64]) -> (impl Display, impl Display) {
    let p1 = part1(input);
    let p2 = part2(input);

    (p1, p2)
}

fn main() {
    let input = get_input_as_int("d10.txt");

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
        let input = "28
33
18
42
31
14
46
20
48
47
24
23
49
45
19
38
39
11
1
32
25
35
8
17
7
9
4
2
34
10
3";

        let input = input
            .split("\n")
            .map(|s| s.parse().unwrap())
            .collect::<Vec<i64>>();
        let res = part1(&input);
        assert_eq!(220, res);
    }

    #[test]
    fn test_p2_1() {
        let input = "16
10
15
5
1
11
7
19
6
12
4";

        let input = input
            .split("\n")
            .map(|s| s.parse().unwrap())
            .collect::<Vec<i64>>();
        let res = part2(&input);
        assert_eq!(8, res);
    }

    #[test]
    fn test_p2_2() {
        let input = "28
33
18
42
31
14
46
20
48
47
24
23
49
45
19
38
39
11
1
32
25
35
8
17
7
9
4
2
34
10
3";

        let input = input
            .split("\n")
            .map(|s| s.parse().unwrap())
            .collect::<Vec<i64>>();
        let res = part2(&input);
        assert_eq!(19208, res);
    }
}
