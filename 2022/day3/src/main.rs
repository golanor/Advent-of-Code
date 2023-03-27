use std::{collections::HashSet, fs};

struct Compartment {
    items: HashSet<String>,
}

impl Compartment {
    fn from_line(line: &str) -> Option<Compartment> {}
}

struct Rucksack {
    compartment_a: Compartment,
    compartment_b: Compartment,
}

fn main() {
    let file_path = "home/or/Code/rust/advent_of_code/2022/day3/input";
    let data = fs::read_to_string(file_path).expect("Unable to read file");
}
