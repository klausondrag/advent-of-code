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
        None => panic!(90920),
        Some((min, max)) => println!("Found in preamble: {} + {} = {}", min, max, min + max),
    }
}

fn solve(input_numbers: Vec<u64>, preamble_length: usize) -> Option<(u64, u64)> {
    let target = find_target(input_numbers.clone(), preamble_length).unwrap();
    for start_index in 0..input_numbers.len() {
        for end_index in start_index + 2..input_numbers.len() {
            let region_of_interest = &input_numbers[start_index..end_index];
            let region_sum: u64 = region_of_interest.iter().sum();
            if region_sum == target {
                let min = *region_of_interest.iter().min().unwrap();
                let max = *region_of_interest.iter().max().unwrap();
                return Some((min, max));
            }
        }
    }

    None
}

fn find_target(input_numbers: Vec<u64>, preamble_length: usize) -> Option<u64> {
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
        let solution = 15 + 47; // = 62
        let output = super::solve(input_numbers, 5);
        match output {
            Some((min, max)) => assert_eq!(solution, min + max),
            None => assert!(false),
        }
    }
}
