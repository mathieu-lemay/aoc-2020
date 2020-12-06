use std::fmt::Display;
use std::time::Instant;

use aoc_2020::get_input;
use std::collections::HashSet;

fn parse_groups_1(lines: &[String]) -> Vec<HashSet<char>> {
    let mut groups: Vec<HashSet<char>> = Vec::new();

    let mut group: HashSet<char> = HashSet::new();
    for l in lines {
        if l.len() == 0 {
            groups.push(group);
            group = HashSet::new();
            continue;
        }

        group.extend(l.chars());
    }

    groups.push(group);

    groups
}

fn parse_groups_2(lines: &[String]) -> Vec<HashSet<char>> {
    let mut groups: Vec<HashSet<char>> = Vec::new();
    let mut is_new = true;

    let mut group: HashSet<char> = HashSet::new();
    for l in lines {
        if l.len() == 0 {
            groups.push(group);
            group = HashSet::new();
            is_new = true;
            continue;
        }

        if is_new {
            is_new = false;
            group.extend(l.chars());
        } else {
            group = group.intersection(&l.chars().collect()).copied().collect();
        }
    }

    groups.push(group);

    groups
}

fn solve(input: &[String]) -> (impl Display, impl Display) {
    let p1 = parse_groups_1(input).iter().map(|g| g.len()).sum::<usize>();
    let p2 = parse_groups_2(input).iter().map(|g| g.len()).sum::<usize>();
    (p1, p2)
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

#[cfg(test)]
mod tests {
    use crate::{parse_groups_1, parse_groups_2};

    #[test]
    fn test_parse_groups_1() {
        let input = "abc

a
b
c

ab
ac

a
a
a
a

b";

        let groups = parse_groups_1(&input.split("\n").map(String::from).collect::<Vec<String>>());
        println!("Groups: {:?}", groups);
        assert_eq!(11_usize, groups.iter().map(|g| g.len()).sum())
    }

    #[test]
    fn test_parse_groups_2() {
        let input = "abc

a
b
c

ab
ac

a
a
a
a

b";

        let groups = parse_groups_2(&input.split("\n").map(String::from).collect::<Vec<String>>());
        println!("Groups: {:?}", groups);
        assert_eq!(6_usize, groups.iter().map(|g| g.len()).sum())
    }
}
