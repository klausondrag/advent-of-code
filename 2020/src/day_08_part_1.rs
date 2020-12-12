use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

enum Command {
    Accumulator,
    Jump,
    NoOperation,
}

struct Operation {
    command: Command,
    argument: i32,
}

pub(crate) fn process(input_filename: &str) {
    let file = File::open(input_filename).unwrap();
    let reader = BufReader::new(file);

    let line_regex = Regex::new(r"^(\w+) (\+|-)(\d+)$").unwrap();
    let mut input = Vec::new();
    for line in reader.lines() {
        let line = line.unwrap();
        for capture in line_regex.captures_iter(&*line) {
            let command = match capture.get(1).unwrap().as_str() {
                "acc" => Command::Accumulator,
                "jmp" => Command::Jump,
                "nop" => Command::NoOperation,
                _ => panic!(90810),
            };
            let sign = match capture.get(2).unwrap().as_str() {
                "+" => 1,
                "-" => -1,
                _ => panic!(90811),
            };
            let number = i32::from_str(capture.get(3).unwrap().as_str()).unwrap();
            let operation = Operation {
                command,
                argument: sign * number,
            };
            input.push(operation);
        }
    }

    let last_value = solve(input);

    println!("Last value in Accumulator: {}", last_value);
}

fn solve(input: Vec<Operation>) -> i32 {
    let mut accumulator = 0;
    let mut is_visited = vec![false; input.len()];

    let mut current_operation_index = 0;
    while !is_visited[current_operation_index] {
        is_visited[current_operation_index] = true;
        current_operation_index = match input[current_operation_index] {
            Operation {
                command: Command::Accumulator,
                argument,
            } => {
                accumulator += argument;
                current_operation_index + 1
            }
            Operation {
                command: Command::Jump,
                argument,
            } => {
                let new_address = (current_operation_index as i32) + argument;
                new_address as usize
            }
            Operation {
                command: Command::NoOperation,
                ..
            } => current_operation_index + 1,
        };
    }

    accumulator
}

#[cfg(test)]
mod tests {
    #[test]
    fn example() {
        let input = vec![
            super::Operation {
                command: super::Command::NoOperation,
                argument: 0,
            },
            super::Operation {
                command: super::Command::Accumulator,
                argument: 1,
            },
            super::Operation {
                command: super::Command::Jump,
                argument: 4,
            },
            super::Operation {
                command: super::Command::Accumulator,
                argument: 3,
            },
            super::Operation {
                command: super::Command::Jump,
                argument: -3,
            },
            super::Operation {
                command: super::Command::Accumulator,
                argument: -99,
            },
            super::Operation {
                command: super::Command::Accumulator,
                argument: 1,
            },
            super::Operation {
                command: super::Command::Jump,
                argument: -4,
            },
            super::Operation {
                command: super::Command::Accumulator,
                argument: 6,
            },
        ];
        let solution = 5;
        let output = super::solve(input);
        assert_eq!(solution, output);
    }
}
