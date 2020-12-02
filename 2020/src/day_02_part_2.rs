use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader};

struct PasswordItem {
    password: String,
    character: char,
    position_1: i32,
    position_2: i32,
}

pub(crate) fn process(input_filename: &str) {
    let file = File::open(input_filename).unwrap();
    let reader = BufReader::new(file);

    let item_regex = Regex::new(r"^(\d+)-(\d+) (\w): (\w+)$").unwrap();
    let mut n_valid_passwords = 0;
    for line in reader.lines() {
        let line = line.unwrap();
        for group in item_regex.captures_iter(&*line) {
            let password_item = PasswordItem {
                password: group[4].parse().unwrap(),
                character: group[3].parse().unwrap(),
                position_1: group[1].parse::<i32>().unwrap(),
                position_2: group[2].parse::<i32>().unwrap(),
            };
            let is_valid = solve(password_item);
            if is_valid {
                n_valid_passwords += 1;
            }
        }
    }

    println!("Found n valid passwords: {}", n_valid_passwords);
}

fn solve(input: PasswordItem) -> bool {
    // - 1 because positions are starting with 1 instead of 0
    let is_character_at_position_1 = input
        .password
        .chars()
        .nth((input.position_1 - 1) as usize)
        .unwrap()
        == input.character;
    let is_character_at_position_2 = input
        .password
        .chars()
        .nth((input.position_2 - 1) as usize)
        .unwrap()
        == input.character;
    is_character_at_position_1 ^ is_character_at_position_2
}

#[cfg(test)]
mod tests {
    #[test]
    fn example() {
        let inputs = vec![
            super::PasswordItem {
                password: String::from("abcde"),
                character: 'a',
                position_1: 1,
                position_2: 3,
            },
            super::PasswordItem {
                password: String::from("cdefg"),
                character: 'b',
                position_1: 1,
                position_2: 3,
            },
            super::PasswordItem {
                password: String::from("ccccccccc"),
                character: 'c',
                position_1: 2,
                position_2: 9,
            },
        ];
        let solutions = vec![true, false, false]; // = 1
        let outputs: Vec<bool> = inputs.into_iter().map(super::solve).collect();
        assert_eq!(solutions, outputs);
    }
}
