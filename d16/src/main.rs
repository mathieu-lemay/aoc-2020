#[macro_use]
extern crate lazy_static;

use std::fmt::Display;
use std::time::Instant;

use regex::Regex;

use aoc_2020::get_input;
use std::collections::{HashMap, HashSet};

lazy_static! {
    static ref FIELD_REGEX: Regex = Regex::new(r"([\w ]+): (\d+)-(\d+) or (\d+)-(\d+)").unwrap();
}

#[derive(Debug, Eq, PartialEq, Hash)]
struct Field {
    name: String,
    range_1: (u32, u32),
    range_2: (u32, u32),
}

impl Field {
    fn is_valid(&self, val: u32) -> bool {
        (val >= self.range_1.0 && val <= self.range_1.1)
            || (val >= self.range_2.0 && val <= self.range_2.1)
    }
}

#[derive(Debug, Eq, PartialEq)]
struct Ticket {
    values: Vec<u32>,
}

#[derive(Debug)]
struct PuzzleData {
    fields: Vec<Field>,
    ticket: Ticket,
    other_tickets: Vec<Ticket>,
}

fn parse(input: &[String]) -> PuzzleData {
    let mut fields = Vec::new();
    let mut ticket = Ticket { values: Vec::new() };
    let mut other_tickets = Vec::new();

    let mut is_my_ticket = false;
    let mut is_other_ticket = false;

    for i in input {
        if is_other_ticket {
            other_tickets.push(Ticket {
                values: i
                    .split(',')
                    .map(|x| x.parse().unwrap())
                    .collect::<Vec<u32>>(),
            });

            continue;
        }

        if is_my_ticket {
            ticket.values = i
                .split(',')
                .map(|x| x.parse().unwrap())
                .collect::<Vec<u32>>();

            is_my_ticket = false;

            continue;
        }

        if i == "your ticket:" {
            is_my_ticket = true;
            continue;
        } else if i == "nearby tickets:" {
            is_other_ticket = true;
            continue;
        }

        if let Some(cap) = FIELD_REGEX.captures(i) {
            let f = Field {
                name: String::from(cap.get(1).unwrap().as_str()),
                range_1: (
                    cap.get(2).unwrap().as_str().parse().unwrap(),
                    cap.get(3).unwrap().as_str().parse().unwrap(),
                ),
                range_2: (
                    cap.get(4).unwrap().as_str().parse().unwrap(),
                    cap.get(5).unwrap().as_str().parse().unwrap(),
                ),
            };

            fields.push(f);
        }
    }

    PuzzleData {
        fields,
        ticket,
        other_tickets,
    }
}

fn get_valid_tickets(input: &PuzzleData) -> (Vec<&Ticket>, u32) {
    let mut values = HashSet::new();
    let mut valid_tickets = Vec::new();

    for f in &input.fields {
        let r = (f.range_1.0..f.range_1.1 + 1)
            .into_iter()
            .collect::<HashSet<u32>>();
        values = values.union(&r).copied().collect();
        let r = (f.range_2.0..f.range_2.1 + 1)
            .into_iter()
            .collect::<HashSet<u32>>();
        values = values.union(&r).copied().collect();
    }

    let mut bad_values = Vec::new();
    for t in &input.other_tickets {
        let mut is_valid = true;
        for i in &t.values {
            if !values.contains(i) {
                is_valid = false;
                bad_values.push(*i);
            }
        }

        if is_valid {
            valid_tickets.push(t);
        }
    }

    (valid_tickets, bad_values.iter().sum::<u32>())
}

fn get_field_names(input: &PuzzleData, valid_tickets: &Vec<&Ticket>) -> Vec<String> {
    let mut possible_fields = input
        .ticket
        .values
        .iter()
        .map(|v| {
            input
                .fields
                .iter()
                .filter(|f| f.is_valid(*v))
                .collect::<Vec<&Field>>()
        })
        .collect::<Vec<Vec<&Field>>>();

    'out: loop {
        for ticket in valid_tickets {
            let taken_fields = possible_fields
                .iter()
                .enumerate()
                .filter(|(idx, fields)| fields.len() == 1)
                .map(|(idx, fields)| (*fields.first().unwrap(), idx))
                .collect::<HashMap<&Field, usize>>();
            if taken_fields.len() == input.fields.len() {
                break 'out;
            }
            possible_fields = possible_fields
                .iter()
                .enumerate()
                .map(|(idx, fields)| {
                    let ticket_val = ticket.values[idx];

                    fields
                        .iter()
                        .filter(|f| {
                            let tf = taken_fields.get(*f);
                            match tf {
                                Some(&i) => i == idx,
                                None => f.is_valid(ticket_val),
                            }
                        })
                        .map(|f| *f)
                        .collect()
                })
                .collect()
        }
    }

    possible_fields
        .iter()
        .map(|fields| fields.first().unwrap().name.clone())
        .collect()
}

fn solve(input: &[String]) -> (impl Display, impl Display) {
    let input = parse(input);
    let (mut valid_tickets, sum_of_bad_values) = get_valid_tickets(&input);
    valid_tickets.push(&input.ticket);
    let field_names = get_field_names(&input, &valid_tickets);

    let p2 = field_names
        .iter()
        .enumerate()
        .map(|(idx, name)| {
            if name.starts_with("departure") {
                input.ticket.values[idx] as u64
            } else {
                1
            }
        })
        .product::<u64>();
    (sum_of_bad_values, p2)
}

fn main() {
    let input = get_input("d16.txt");

    let start = Instant::now();

    let (r1, r2) = solve(input.as_slice());

    let t = start.elapsed().as_micros() as f64 / 1000.0;

    println!("Part 1: {}", r1);
    println!("Part 2: {}", r2);
    println!("Duration: {:.3}ms", t);
}

#[cfg(test)]
mod tests {
    use crate::{get_field_names, get_valid_tickets, parse, Ticket};

    #[test]
    fn test_p1() {
        let input = "class: 1-3 or 5-7
row: 6-11 or 33-44
seat: 13-40 or 45-50

your ticket:
7,1,14

nearby tickets:
7,3,47
40,4,50
55,2,20
38,6,12";

        let input = input.split("\n").map(String::from).collect::<Vec<String>>();
        let values = parse(&input);
        let (valid_tickets, res) = get_valid_tickets(&values);

        assert_eq!(
            vec![&Ticket {
                values: vec![7, 3, 47]
            }],
            valid_tickets
        );
        assert_eq!(71, res);
    }

    #[test]
    fn test_p2() {
        let input = "class: 0-1 or 4-19
row: 0-5 or 8-19
seat: 0-13 or 16-19

your ticket:
11,12,13

nearby tickets:
3,9,18
15,1,5
5,14,9";

        let input = input.split("\n").map(String::from).collect::<Vec<String>>();
        let values = parse(&input);
        let (valid_tickets, _) = get_valid_tickets(&values);
        let field_names = get_field_names(&values, &valid_tickets);

        assert_eq!(vec!["row", "class", "seat"], field_names);
    }
}
