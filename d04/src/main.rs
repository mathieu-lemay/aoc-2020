#[macro_use]
extern crate lazy_static;

use std::fmt::Display;
use std::time::Instant;

use regex::Regex;

use aoc_2020::get_input;

lazy_static! {
    static ref HGT_RGX: Regex = Regex::new(r"(\d+)(cm|in)").unwrap();
    static ref HCL_RGX: Regex = Regex::new(r"#[0-9a-f]{6}").unwrap();
    static ref VALID_ECL: Vec<&'static str> = vec!["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];
}

#[derive(Debug, Default)]
struct Passport {
    byr: Option<String>,
    iyr: Option<String>,
    eyr: Option<String>,
    hgt: Option<String>,
    hcl: Option<String>,
    ecl: Option<String>,
    pid: Option<String>,
    cid: Option<String>,
}

impl Passport {
    fn has_all_fields(&self) -> bool {
        self.byr.is_some()
            && self.iyr.is_some()
            && self.eyr.is_some()
            && self.hgt.is_some()
            && self.hcl.is_some()
            && self.ecl.is_some()
            && self.pid.is_some()
    }

    fn is_valid(&self) -> bool {
        if !self.has_all_fields() {
            return false;
        }

        let byr = self.byr.as_ref().unwrap();
        if byr.len() != 4 {
            return false;
        }
        match byr.parse::<i32>() {
            Ok(y) => {
                if y < 1920 || y > 2002 {
                    return false;
                }
            }
            Err(_) => return false,
        }

        let iyr = self.iyr.as_ref().unwrap();
        if iyr.len() != 4 {
            return false;
        }
        match iyr.parse::<i32>() {
            Ok(y) => {
                if y < 2010 || y > 2020 {
                    return false;
                }
            }
            Err(_) => return false,
        }

        let eyr = self.eyr.as_ref().unwrap();
        if eyr.len() != 4 {
            return false;
        }
        match eyr.parse::<i32>() {
            Ok(y) => {
                if y < 2020 || y > 2030 {
                    return false;
                }
            }
            Err(_) => return false,
        }

        let cap = HGT_RGX.captures(self.hgt.as_ref().unwrap());
        match cap {
            Some(cap) => {
                let h = cap.get(1).unwrap().as_str().parse::<i32>().unwrap();
                match cap.get(2).unwrap().as_str() {
                    "cm" => {
                        if h < 150 || h > 193 {
                            return false;
                        }
                    }
                    "in" => {
                        if h < 59 || h > 76 {
                            return false;
                        }
                    }
                    _ => panic!("Wut?"),
                }
            }
            None => return false,
        }

        if !HCL_RGX.is_match(self.hcl.as_ref().unwrap()) {
            return false;
        }

        if !VALID_ECL.contains(&self.ecl.as_ref().unwrap().as_str()) {
            return false;
        }

        let pid = self.pid.as_ref().unwrap();
        if pid.len() != 9 {
            return false;
        }
        if let Err(_) = pid.parse::<i32>() {
            return false;
        }

        true
    }
}

fn parse_passports(lines: &[String]) -> Vec<Passport> {
    let mut passports: Vec<Passport> = Vec::new();

    let mut byr: Option<String> = None;
    let mut iyr: Option<String> = None;
    let mut eyr: Option<String> = None;
    let mut hgt: Option<String> = None;
    let mut hcl: Option<String> = None;
    let mut ecl: Option<String> = None;
    let mut pid: Option<String> = None;
    let mut cid: Option<String> = None;

    for line in lines {
        if line.len() == 0 {
            let pp = Passport {
                byr: byr.clone(),
                iyr: iyr.clone(),
                eyr: eyr.clone(),
                hgt: hgt.clone(),
                hcl: hcl.clone(),
                ecl: ecl.clone(),
                pid: pid.clone(),
                cid: cid.clone(),
            };

            passports.push(pp);

            byr = None;
            iyr = None;
            eyr = None;
            hgt = None;
            hcl = None;
            ecl = None;
            pid = None;
            cid = None;
        } else {
            let entries = line.split(' ');
            for e in entries {
                let e = e.splitn(2, ':').collect::<Vec<&str>>();
                let k = *e.get(0).unwrap();
                let v = *e.get(1).unwrap();

                match k {
                    "byr" => byr = Some(String::from(v)),
                    "iyr" => iyr = Some(String::from(v)),
                    "eyr" => eyr = Some(String::from(v)),
                    "hgt" => hgt = Some(String::from(v)),
                    "hcl" => hcl = Some(String::from(v)),
                    "ecl" => ecl = Some(String::from(v)),
                    "pid" => pid = Some(String::from(v)),
                    "cid" => cid = Some(String::from(v)),
                    _ => panic!("Invalid key: {}", k),
                }
            }
        }
    }

    let pp = Passport {
        byr: byr.clone(),
        iyr: iyr.clone(),
        eyr: eyr.clone(),
        hgt: hgt.clone(),
        hcl: hcl.clone(),
        ecl: ecl.clone(),
        pid: pid.clone(),
        cid: cid.clone(),
    };

    passports.push(pp);

    passports
}

fn solve(input: &[String]) -> (impl Display, impl Display) {
    let passports = parse_passports(input);

    let nb_valid1 = passports.iter().filter(|pp| pp.has_all_fields()).count();
    let nb_valid2 = passports.iter().filter(|pp| pp.is_valid()).count();

    (nb_valid1, nb_valid2)
}

fn main() {
    let input = get_input("d04.txt");

    let start = Instant::now();

    let (r1, r2) = solve(input.as_slice());

    let t = start.elapsed().as_micros() as f64 / 1000.0;

    println!("Part 1: {}", r1);
    println!("Part 2: {}", r2);
    println!("Duration: {:.3}ms", t);
}
