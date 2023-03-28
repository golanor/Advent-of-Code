use std::{collections::HashSet, fs};

struct Compartment {
    items: HashSet<String>,
}

impl Compartment {
    fn new(line: &str) -> Compartment {
        let items = line.split_whitespace().map(|s| s.to_string()).collect();
        Compartment { items }
    }
}

struct Rucksack {
    compartment_a: Compartment,
    compartment_b: Compartment,
}


impl Rucksack {
    fn from_str(line: &str) -> Rucksack {
        let line_length = line.len();
        let (compartment_a, compartment_b) = line.split_at(line_length / 2);
        Rucksack {
            compartment_a: Compartment::new(compartment_a),
            compartment_b: Compartment::new(compartment_b),
        }
    }

    fn find_shared_items(&self) -> HashSet<String> {
        self.compartment_a.items.intersection(&self.compartment_b.items).map(|s| s.to_string()).collect()
    }
}

fn item_score(item: &String) -> u32 {
    item.chars().map(|c| c as u32 - 96).sum()
}



fn main() {
    let file_path = "/home/orgolan/Code/rust/Advent-o5f-Code/2022/day3/input";
    let data = fs::read_to_string(file_path).expect("Unable to read file");
    let rucksacks: Vec<Rucksack> = data.lines().map(|line| Rucksack::from_str(line)).collect();
    let total_score: u32 = rucksacks.iter().map(|r| r.find_shared_items().iter().map(|i| item_score(i)).sum::<u32>()).sum();
    println!("{}", total_score);
}
