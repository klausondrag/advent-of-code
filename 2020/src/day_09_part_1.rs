use std::fs::File;
use std::io::{BufRead, BufReader};

pub(crate) fn process(input_filename: &str) {
    let file = File::open(input_filename).unwrap();
    let reader = BufReader::new(file);

    let mut input_numbers = Vec::new();
    for line in reader.lines() {
        let line = line.unwrap();
        let input_number = line.parse::<u64>().unwrap();
        input_numbers.push(input_number);
    }

    match solve(input_numbers, 25) {
        None => panic!(90910),
        Some(first_invalid_number) => println!("First invalid number: {}", first_invalid_number),
    }
}

fn solve(input_numbers: Vec<u64>, preamble_length: usize) -> Option<u64> {
    for index_of_interest in preamble_length..input_numbers.len() {
        let start_index = (index_of_interest - preamble_length) as usize;
        let end_index = index_of_interest as usize;
        let preamble = &input_numbers[start_index..end_index];
        let target = input_numbers[end_index];
        if !is_valid_target(preamble, target) {
            return Some(target);
        }
    }

    None
}

fn is_valid_target(preamble: &[u64], target: u64) -> bool {
    for a in preamble.iter() {
        for b in preamble.iter() {
            if *a + *b == target {
                return true;
            }
        }
    }

    false
}

#[cfg(test)]
mod tests {
    #[test]
    fn example() {
        let input_numbers = vec![
            35, 20, 15, 25, 47, 40, 62, 55, 65, 95, 102, 117, 150, 182, 127, 219, 299, 277, 309,
            576,
        ];
        let solution = 127;
        let output = super::solve(input_numbers, 5);
        match output {
            Some(output) => assert_eq!(solution, output),
            None => assert!(false),
        }
    }
}
