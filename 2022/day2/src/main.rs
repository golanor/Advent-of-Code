use std::fs;

enum OpponentChoice {
    A,
    B,
    C,
}

enum PlayerChoice {
    X,
    Y,
    Z,
}

struct Row {
    op_choice: OpponentChoice,
    player_choice: PlayerChoice,
}

impl Row {
    fn parse(line: &str) -> Option<Row> {
        if line.contains("A") {
            if line.contains("X") {
                Some(Row {
                    op_choice: OpponentChoice::A,
                    player_choice: PlayerChoice::X,
                })
            } else if line.contains("Y") {
                Some(Row {
                    op_choice: OpponentChoice::A,
                    player_choice: PlayerChoice::Y,
                })
            } else if line.contains("Z") {
                Some(Row {
                    op_choice: OpponentChoice::A,
                    player_choice: PlayerChoice::Z,
                })
            } else {
                None
            }
        } else if line.contains("B") {
            if line.contains("X") {
                Some(Row {
                    op_choice: OpponentChoice::B,
                    player_choice: PlayerChoice::X,
                })
            } else if line.contains("Y") {
                Some(Row {
                    op_choice: OpponentChoice::B,
                    player_choice: PlayerChoice::Y,
                })
            } else if line.contains("Z") {
                Some(Row {
                    op_choice: OpponentChoice::B,
                    player_choice: PlayerChoice::Z,
                })
            } else {
                None
            }
        } else if line.contains("C") {
            if line.contains("X") {
                Some(Row {
                    op_choice: OpponentChoice::C,
                    player_choice: PlayerChoice::X,
                })
            } else if line.contains("Y") {
                Some(Row {
                    op_choice: OpponentChoice::C,
                    player_choice: PlayerChoice::Y,
                })
            } else if line.contains("Z") {
                Some(Row {
                    op_choice: OpponentChoice::C,
                    player_choice: PlayerChoice::Z,
                })
            } else {
                None
            }
        } else {
            None
        }
    }

    fn score(&self) -> u32 {
        use OpponentChoice::*;
        use PlayerChoice::*;
        match self {
            Row {
                op_choice: A,
                player_choice: x,
            } => match x {
                X => 3,
                Y => 4,
                Z => 8,
            },
            Row {
                op_choice: B,
                player_choice: x,
            } => match x {
                X => 1,
                Y => 5,
                Z => 9,
            },
            Row {
                op_choice: C,
                player_choice: x,
            } => match x {
                X => 2,
                Y => 6,
                Z => 7,
            },
        }
    }
}

fn main() {
    let file_path = "/home/or/Code/rust/advent_of_code/2022/day2/input";
    let data = fs::read_to_string(file_path).expect("Unable to read file");
    println!(
        "The score is {:?}",
        data.split("\n")
            .into_iter()
            .map(Row::parse)
            .flatten()
            .map(|x| x.score())
            .sum::<u32>()
    )
}
