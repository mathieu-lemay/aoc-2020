use std::fmt::Display;
use std::str::FromStr;
use std::time::Instant;

use aoc_2020::get_input;

enum CardinalDirection {
    North,
    South,
    East,
    West,
}

impl CardinalDirection {
    fn get_translation_vector(&self, count: u32) -> (i32, i32) {
        match self {
            Self::North => (0, count as i32),
            Self::South => (0, -(count as i32)),
            Self::East => (count as i32, 0),
            Self::West => (-(count as i32), 0),
        }
    }
}

struct ParseInstructionError {}

enum Instruction {
    North(u32),
    South(u32),
    East(u32),
    West(u32),
    Left(u32),
    Right(u32),
    Forward(u32),
}

impl FromStr for Instruction {
    type Err = ParseInstructionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut chars = s.chars();
        let instr = match chars.next() {
            Some(c) => match c {
                'N' => Instruction::North,
                'S' => Instruction::South,
                'E' => Instruction::East,
                'W' => Instruction::West,
                'L' => Instruction::Left,
                'R' => Instruction::Right,
                'F' => Instruction::Forward,
                _ => return Err(ParseInstructionError {}),
            },
            None => return Err(ParseInstructionError {}),
        };

        match chars.as_str().parse::<u32>() {
            Ok(c) => Ok(instr(c)),
            Err(_) => Err(ParseInstructionError {}),
        }
    }
}

struct Ship {
    pos_x: i32,
    pos_y: i32,
    way_x: i32,
    way_y: i32,
}

impl Ship {
    fn new(waypoint: (i32, i32)) -> Self {
        Ship {
            pos_x: 0,
            pos_y: 0,
            way_x: waypoint.0,
            way_y: waypoint.1,
        }
    }

    fn move_ship(&mut self, vec: (i32, i32)) {
        self.pos_x += vec.0;
        self.pos_y += vec.1;
    }

    fn move_waypoint(&mut self, vec: (i32, i32)) {
        self.way_x += vec.0;
        self.way_y += vec.1;
    }

    fn rotate_waypoint_left(&mut self, deg: i32) {
        let waypoint = rotate_vec((self.way_x, self.way_y), deg);
        self.way_x = waypoint.0;
        self.way_y = waypoint.1;
    }

    fn rotate_waypoint_right(&mut self, deg: i32) {
        self.rotate_waypoint_left(-deg);
    }

    fn apply_instruction(&mut self, instr: &Instruction, move_fn: &dyn Fn(&mut Self, (i32, i32))) {
        match instr {
            Instruction::North(c) => {
                move_fn(self, CardinalDirection::North.get_translation_vector(*c))
            }
            Instruction::South(c) => {
                move_fn(self, CardinalDirection::South.get_translation_vector(*c))
            }
            Instruction::East(c) => {
                move_fn(self, CardinalDirection::East.get_translation_vector(*c))
            }
            Instruction::West(c) => {
                move_fn(self, CardinalDirection::West.get_translation_vector(*c))
            }
            Instruction::Left(c) => self.rotate_waypoint_left(*c as i32),
            Instruction::Right(c) => self.rotate_waypoint_right(*c as i32),
            Instruction::Forward(c) => {
                let c = *c as i32;
                self.move_ship((self.way_x * c, self.way_y * c));
            }
        };
    }
}

fn rotate_vec(vector: (i32, i32), deg: i32) -> (i32, i32) {
    let angle = (deg as f64).to_radians();

    let x = vector.0 as f64;
    let y = vector.1 as f64;

    let new_x = (angle.cos() * x - angle.sin() * y).round() as i32;
    let new_y = (angle.sin() * x + angle.cos() * y).round() as i32;

    (new_x, new_y)
}

fn parse(input: &[String]) -> Vec<Instruction> {
    input
        .iter()
        .map(|i| match Instruction::from_str(i.as_str()) {
            Ok(i) => i,
            Err(_) => panic!("Unable to parse {}", i),
        })
        .collect()
}

fn part1(instr: &Vec<Instruction>) -> u32 {
    let mut ship = Ship::new((1, 0));

    for i in instr {
        ship.apply_instruction(i, &Ship::move_ship);
    }

    (ship.pos_x.abs() + ship.pos_y.abs()) as u32
}

fn part2(instr: &Vec<Instruction>) -> u32 {
    let mut ship = Ship::new((10, 1));

    for i in instr {
        ship.apply_instruction(i, &Ship::move_waypoint);
    }

    (ship.pos_x.abs() + ship.pos_y.abs()) as u32
}

fn solve(input: &[String]) -> (impl Display, impl Display) {
    let instr = parse(input);
    let p1 = part1(&instr);
    let p2 = part2(&instr);

    (p1, p2)
}

fn main() {
    let input = get_input("d12.txt");

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
        let input = "F10
N3
F7
R90
F11";

        let input = input.split("\n").map(String::from).collect::<Vec<String>>();
        let res = part1(&parse(&input));

        assert_eq!(25, res);
    }

    #[test]
    fn test_p2() {
        let input = "F10
N3
F7
R90
F11";

        let input = input.split("\n").map(String::from).collect::<Vec<String>>();
        let res = part2(&parse(&input));

        assert_eq!(286, res);
    }
}
