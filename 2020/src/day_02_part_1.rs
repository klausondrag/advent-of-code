use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader};

struct PasswordItem {
    password: String,
    character: char,
    min_occurrence: i32,
    max_occurrence: i32,
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
                min_occurrence: group[1].parse::<i32>().unwrap(),
                max_occurrence: group[2].parse::<i32>().unwrap(),
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
    let mut char_counter = 0;
    for c in input.password.chars() {
        if c == input.character {
            char_counter += 1;
        }
    }
    input.min_occurrence <= char_counter && char_counter <= input.max_occurrence
}

#[cfg(test)]
mod tests {
    #[test]
    fn example() {
        let inputs = vec![
            super::PasswordItem {
                password: String::from("abcde"),
                character: 'a',
                min_occurrence: 1,
                max_occurrence: 3,
            },
            super::PasswordItem {
                password: String::from("cdefg"),
                character: 'b',
                min_occurrence: 1,
                max_occurrence: 3,
            },
            super::PasswordItem {
                password: String::from("ccccccccc"),
                character: 'c',
                min_occurrence: 2,
                max_occurrence: 9,
            },
        ];
        let solutions = vec![true, false, true]; // = 2
        let outputs: Vec<bool> = inputs.into_iter().map(super::solve).collect();
        assert_eq!(solutions, outputs);
    }
}
