/// This files contains the solution for the first day of the Advent of Code 2022.

mod day1 {
    use itertools::Itertools;
use std::fs;
use std::str::FromStr;

fn day1() {
    let file_path = "/home/or/Code/rust/advent_of_code/2022/day1/input";
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let mut calories = vec![];
    for group in contents.split("\n").group_by(|x| x.is_empty()).into_iter() {
        if !group.0 {
            calories.push(group.1.map(|x| u64::from_str(x).unwrap()).sum::<u64>());
        }
    }
    calories.sort();
    calories.reverse();
    println!(
        "Calories of top 3 elves: {:?}",
        calories.into_iter().take(3).sum::<u64>()
    )
}
}
