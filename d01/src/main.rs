use aoc_2020::get_input_as_int;
use itertools::Itertools;

fn main() {
    let input = get_input_as_int("d01.txt");

    for values in input.iter().combinations(2) {
        if values.iter().map(|i| *i).sum::<i32>() == 2020 {
            println!("Part 1: {}", values.into_iter().product::<i32>());
            break;
        }
    }

    for values in input.iter().combinations(3) {
        if values.iter().map(|i| *i).sum::<i32>() == 2020 {
            println!("Part 2: {}", values.into_iter().product::<i32>());
            break;
        }
    }
}
