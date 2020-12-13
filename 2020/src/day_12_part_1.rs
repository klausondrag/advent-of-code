use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

#[derive(Clone, Copy)]
enum Action {
    North,
    South,
    East,
    West,
    Left,
    Right,
    Forward,
}

struct Instruction {
    action: Action,
    number: i32,
}

#[derive(Clone, Copy)]
struct Status {
    x: i32,
    y: i32,
    action: Action,
}

pub(crate) fn process(input_filename: &str) {
    let file = File::open(input_filename).unwrap();
    let reader = BufReader::new(file);

    let line_regex = Regex::new(r"^(\w)(\d+)$").unwrap();
    let mut input = Vec::new();
    for line in reader.lines() {
        let line = line.unwrap();
        for capture in line_regex.captures_iter(&*line) {
            let direction = match capture.get(1).unwrap().as_str() {
                "N" => Action::North,
                "S" => Action::South,
                "E" => Action::East,
                "W" => Action::West,
                "L" => Action::Left,
                "R" => Action::Right,
                "F" => Action::Forward,
                _ => panic!(91210),
            };
            let number = i32::from_str(capture.get(2).unwrap().as_str()).unwrap();
            let instruction = Instruction {
                action: direction,
                number,
            };
            input.push(instruction);
        }
    }

    let (north_south_position, east_west_position) = solve(input);

    println!(
        "Final location:\nNorth/South: {}\nEast/West: {}\nDistance: {}",
        north_south_position,
        east_west_position,
        north_south_position.abs() + east_west_position.abs()
    );
}

fn solve(input: Vec<Instruction>) -> (i32, i32) {
    let mut status = Status {
        x: 0,
        y: 0,
        action: Action::East,
    };

    for instruction in input {
        status = process_instruction(status, instruction);
    }

    (status.x, status.y)
}

fn process_instruction(status: Status, instruction: Instruction) -> Status {
    match instruction {
        Instruction {
            action: Action::North,
            number,
        } => Status {
            y: status.y + number,
            ..status
        },
        Instruction {
            action: Action::South,
            number,
        } => Status {
            y: status.y - number,
            ..status
        },
        Instruction {
            action: Action::East,
            number,
        } => Status {
            x: status.x + number,
            ..status
        },
        Instruction {
            action: Action::West,
            number,
        } => Status {
            x: status.x - number,
            ..status
        },
        Instruction {
            action: Action::Left,
            number: 90,
        } => Status {
            action: turn_multiple_left(status.action, 1),
            ..status
        },
        Instruction {
            action: Action::Left,
            number: 180,
        } => Status {
            action: turn_multiple_left(status.action, 2),
            ..status
        },
        Instruction {
            action: Action::Left,
            number: 270,
        } => Status {
            action: turn_multiple_left(status.action, 3),
            ..status
        },
        Instruction {
            action: Action::Right,
            number: 90,
        } => Status {
            action: turn_multiple_left(status.action, 3),
            ..status
        },
        Instruction {
            action: Action::Right,
            number: 180,
        } => Status {
            action: turn_multiple_left(status.action, 2),
            ..status
        },
        Instruction {
            action: Action::Right,
            number: 270,
        } => Status {
            action: turn_multiple_left(status.action, 1),
            ..status
        },
        Instruction {
            action: Action::Forward,
            number,
        } => process_instruction(
            status,
            Instruction {
                action: status.action,
                number,
            },
        ),
        _ => panic!(91211),
    }
}

fn turn_multiple_left(start_action: Action, n: u8) -> Action {
    let mut new_action = start_action;
    for _ in 0..n {
        new_action = turn_left_90(new_action);
    }
    new_action
}

fn turn_left_90(current_direction: Action) -> Action {
    match current_direction {
        Action::North => Action::West,
        Action::South => Action::East,
        Action::East => Action::North,
        Action::West => Action::South,
        _ => panic!(91212),
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn example() {
        let input = vec![
            super::Instruction {
                action: super::Action::Forward,
                number: 10,
            },
            super::Instruction {
                action: super::Action::North,
                number: 3,
            },
            super::Instruction {
                action: super::Action::Forward,
                number: 7,
            },
            super::Instruction {
                action: super::Action::Right,
                number: 90,
            },
            super::Instruction {
                action: super::Action::Forward,
                number: 11,
            },
        ];
        let solution = 17 + 8; // = 25
        let (north_south_position, east_west_position) = super::solve(input);
        assert_eq!(
            solution,
            north_south_position.abs() + east_west_position.abs()
        );
    }
}
