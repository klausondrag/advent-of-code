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
    ship_x: i32,
    ship_y: i32,
    waypoint_x: i32,
    waypoint_y: i32,
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
                _ => panic!(91220),
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
        ship_x: 0,
        ship_y: 0,
        waypoint_x: 10,
        waypoint_y: 1,
    };

    for instruction in input {
        status = process_instruction(status, instruction);
    }

    (status.ship_x, status.ship_y)
}

fn process_instruction(status: Status, instruction: Instruction) -> Status {
    match instruction {
        Instruction {
            action: Action::North,
            number,
        } => Status {
            waypoint_y: status.waypoint_y + number,
            ..status
        },
        Instruction {
            action: Action::South,
            number,
        } => Status {
            waypoint_y: status.waypoint_y - number,
            ..status
        },
        Instruction {
            action: Action::East,
            number,
        } => Status {
            waypoint_x: status.waypoint_x + number,
            ..status
        },
        Instruction {
            action: Action::West,
            number,
        } => Status {
            waypoint_x: status.waypoint_x - number,
            ..status
        },
        Instruction {
            action: Action::Left,
            number: 90,
        } => turn_multiple_left(status, 1),
        Instruction {
            action: Action::Left,
            number: 180,
        } => turn_multiple_left(status, 2),
        Instruction {
            action: Action::Left,
            number: 270,
        } => turn_multiple_left(status, 3),
        Instruction {
            action: Action::Right,
            number: 90,
        } => turn_multiple_left(status, 3),
        Instruction {
            action: Action::Right,
            number: 180,
        } => turn_multiple_left(status, 2),
        Instruction {
            action: Action::Right,
            number: 270,
        } => turn_multiple_left(status, 1),
        Instruction {
            action: Action::Forward,
            number,
        } => Status {
            ship_x: status.ship_x + number * status.waypoint_x,
            ship_y: status.ship_y + number * status.waypoint_y,
            ..status
        },

        _ => panic!(91221),
    }
}

fn turn_multiple_left(start_status: Status, n: u8) -> Status {
    let mut new_status = start_status.clone();
    for _ in 0..n {
        new_status = turn_left_90(new_status);
    }
    new_status
}

fn turn_left_90(status: Status) -> Status {
    Status {
        waypoint_x: -status.waypoint_y,
        waypoint_y: status.waypoint_x,
        ..status
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
        let solution = 214 + 72; // = 286
        let (north_south_position, east_west_position) = super::solve(input);
        assert_eq!(
            solution,
            north_south_position.abs() + east_west_position.abs()
        );
    }
}
