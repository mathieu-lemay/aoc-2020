use num::integer::lcm;
use std::fmt::Display;
use std::time::Instant;

use aoc_2020::get_input;

#[derive(Debug)]
struct Bus {
    number: u64,
    offset: u64,
}

fn parse(input: &[String]) -> (u64, Vec<Bus>) {
    let ts = input[0].parse().unwrap();
    let buses = input[1]
        .split(',')
        .enumerate()
        .map(|(idx, num)| (idx, num.parse()))
        .filter(|(_, num)| num.is_ok())
        .map(|(idx, num)| Bus {
            number: num.unwrap(),
            offset: idx as u64,
        })
        .collect();

    (ts, buses)
}

fn part1(ts: u64, buses: &Vec<Bus>) -> u64 {
    let bus_wait_times = buses
        .iter()
        .map(|b| (b.number, b.number - ts % b.number))
        .collect::<Vec<(u64, u64)>>();

    let next_bus = bus_wait_times.iter().min_by_key(|b| b.1).unwrap();

    next_bus.0 * next_bus.1
}

fn sync(period1: u64, phase1: u64, period2: u64, phase2: u64) -> (u64, u64) {
    let new_period = lcm(period1, period2);

    for i in 0..(new_period / period1) {
        let n = i * period1 + phase1;
        if n % period2 == phase2 {
            return (new_period, n);
        }
    }

    panic!("Impossible to sync")
}

fn get_phase(number: i64, offset: i64) -> u64 {
    let mut n = number - offset;

    while n < offset {
        n += number;
    }

    (n % number) as u64
}

fn part2(buses: &Vec<Bus>) -> u64 {
    let mut period = buses[0].number;
    let mut phase = buses[0].offset;

    for b in buses.iter().skip(1) {
        let r = sync(
            period,
            phase,
            b.number,
            get_phase(b.number as i64, b.offset as i64),
        );
        period = r.0;
        phase = r.1;
    }

    phase as u64
}

fn solve(input: &[String]) -> (impl Display, impl Display) {
    let (ts, buses) = parse(input);
    let p1 = part1(ts, &buses);
    let p2 = part2(&buses);

    println!("{}", p2 as f64 / 89308340866483f64);

    (p1, p2)
}

fn main() {
    let input = get_input("d13.txt");

    let start = Instant::now();

    let (r1, r2) = solve(input.as_slice());

    let t = start.elapsed().as_micros() as f64 / 1000.0;

    println!("Part 1: {}", r1);
    println!("Part 2: {}", r2);
    println!("Duration: {:.3}ms", t);
}

#[cfg(test)]
mod tests {
    use crate::{parse, part1, part2, sync};

    #[test]
    fn test_p1() {
        let input = "939
7,13,x,x,59,x,31,19";

        let input = input.split("\n").map(String::from).collect::<Vec<String>>();
        let (ts, buses) = parse(&input);
        let res = part1(ts, &buses);

        assert_eq!(295, res);

        let input = "1008832
23,x,x,x,x,x,x,x,x,x,x,x,x,41,x,x,x,x,x,x,x,x,x,449,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,13,19,x,x,x,x,x,x,x,x,x,29,x,991,x,x,x,x,x,37,x,x,x,x,x,x,x,x,x,x,17";

        let input = input.split("\n").map(String::from).collect::<Vec<String>>();
        let (ts, buses) = parse(&input);
        let res = part1(ts, &buses);

        assert_eq!(5946, res);
    }

    #[test]
    fn test_p2() {
        let input = "939
7,13,x,x,59,x,31,19";

        let input = input.split("\n").map(String::from).collect::<Vec<String>>();
        let (_, buses) = parse(&input);
        let res = part2(&buses);

        assert_eq!(1068781, res);

        let input = "1008832
23,x,x,x,x,x,x,x,x,x,x,x,x,41,x,x,x,x,x,x,x,x,x,449,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,13,19,x,x,x,x,x,x,x,x,x,29,x,991,x,x,x,x,x,37,x,x,x,x,x,x,x,x,x,x,17";

        let input = input.split("\n").map(String::from).collect::<Vec<String>>();
        let (_, buses) = parse(&input);
        let res = part2(&buses);

        assert_eq!(645338524823718, res);
    }

    #[test]
    fn test_sync() {
        assert_eq!((91, 77), sync(7, 0, 13, 12));
        assert_eq!((45, 18), sync(9, 0, 5, 3));
        assert_eq!((570, 120), sync(30, 0, 38, 6));
        assert_eq!((570, 120), sync(38, 6, 30, 0));
    }
}
