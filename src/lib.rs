use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn get_input(filename: &str) -> Vec<String> {
    let file = match File::open(format!("input/{}", filename)) {
        Ok(file) => file,
        Err(error) => panic!("Unable to open file {}: {}", filename, error),
    };

    let reader = BufReader::new(file);

    reader.lines().map(|l| l.unwrap()).collect()
}

pub fn get_input_as_int(filename: &str) -> Vec<i32> {
    get_input(filename)
        .iter()
        .map(|i| i.parse().unwrap())
        .collect()
}
