use std::fmt::Display;
use std::time::Instant;

use aoc_2020::get_input;
use std::collections::HashMap;

#[derive(Debug)]
struct Bag {
    color: String,
    contents: Vec<Contain>,
}

#[derive(Debug)]
struct Contain {
    amount: u32,
    color: String,
}

fn parse(input: &[String]) -> HashMap<String, Bag> {
    let mut bags = HashMap::new();

    for l in input {
        let mut s = l.split(" bags contain ");
        let mut container = s.next().unwrap().split(" ");

        let mut bag = Bag {
            color: format!(
                "{} {}",
                container.next().unwrap(),
                container.next().unwrap()
            ),
            contents: Vec::new(),
        };

        let contents = s.next().unwrap();
        if contents == "no other bags." {
            bags.insert(bag.color.clone(), bag);

            continue;
        }

        let contents = contents.split(", ");
        for c in contents {
            let mut b = c.split(" ");

            let c2 = Contain {
                amount: b.next().unwrap().parse().unwrap(),
                color: format!("{} {}", b.next().unwrap(), b.next().unwrap()),
            };

            bag.contents.push(c2);
        }

        bags.insert(bag.color.clone(), bag);
    }

    bags
}

fn can_contain_shiny_gold(bag: &Bag, bags: &HashMap<String, Bag>) -> bool {
    for c in &bag.contents {
        if c.color == "shiny gold" {
            return true;
        }

        if can_contain_shiny_gold(&bags.get(&c.color).unwrap(), &bags) {
            return true;
        }
    }

    false
}

fn part1(bags: &HashMap<String, Bag>) -> usize {
    bags.values()
        .filter(|b| can_contain_shiny_gold(b, bags))
        .count()
}

fn count_contents(bag: &Bag, bags: &HashMap<String, Bag>) -> u32 {
    let mut count = 0u32;

    for c in &bag.contents {
        count += c.amount;

        count += c.amount * count_contents(bags.get(&c.color).unwrap(), bags);
    }

    count
}

fn part2(bags: &HashMap<String, Bag>) -> u32 {
    let mybag = bags.get("shiny gold").unwrap();

    count_contents(mybag, bags)
}

fn solve(input: &[String]) -> (impl Display, impl Display) {
    let bags = parse(input);

    let p1 = part1(&bags);
    let p2 = part2(&bags);

    (p1, p2)
}

fn main() {
    let input = get_input("d07.txt");

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
        let input = "light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.";

        let bags = parse(&input.split("\n").map(String::from).collect::<Vec<String>>());
        let res = part1(&bags);
        assert_eq!(4, res);
    }

    #[test]
    fn test_p2_1() {
        let input = "light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.";

        let bags = parse(&input.split("\n").map(String::from).collect::<Vec<String>>());
        let res = part2(&bags);
        assert_eq!(32, res);
    }

    #[test]
    fn test_p2_2() {
        let input = "shiny gold bags contain 2 dark red bags.
dark red bags contain 2 dark orange bags.
dark orange bags contain 2 dark yellow bags.
dark yellow bags contain 2 dark green bags.
dark green bags contain 2 dark blue bags.
dark blue bags contain 2 dark violet bags.
dark violet bags contain no other bags.";

        let bags = parse(&input.split("\n").map(String::from).collect::<Vec<String>>());
        let res = part2(&bags);
        assert_eq!(126, res);
    }
}
