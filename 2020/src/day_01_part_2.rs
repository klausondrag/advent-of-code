use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::process::exit;

pub(crate) fn process(input_filename: &str) {
    let file = File::open(input_filename).unwrap();
    let reader = BufReader::new(file);

    let mut input_numbers = Vec::new();
    for line in reader.lines() {
        let line = line.unwrap();
        let input_number = line.parse::<i32>().unwrap();
        input_numbers.push(input_number);
    }

    const TARGET_SUM: i32 = 2020;
    let solution = solve(input_numbers, TARGET_SUM);

    match solution {
        Some((a, b, c)) => {
            let result = a * b * c;
            println!(
                "Found Numbers.\n{} + {} + {} = {}\n{} * {} * {} = {}",
                a, b, c, TARGET_SUM, a, b, c, result
            );
        }
        None => {
            println!("Error: No numbers adding up to {} found!", TARGET_SUM);
            // error specification: 9XXYZ for the Z-th error of day XX part Y
            exit(90120);
        }
    }
}

fn solve(input_numbers: Vec<i32>, target_sum: i32) -> Option<(i32, i32, i32)> {
    let mut round_1: HashMap<i32, i32> = HashMap::new();
    for round_1_number in &input_numbers {
        let round_1_difference = target_sum - *round_1_number;
        round_1.insert(round_1_difference, *round_1_number);
    }

    let mut round_2: HashMap<i32, (i32, i32)> = HashMap::new();
    for round_2_number in &input_numbers {
        let missing_number = round_2.get(round_2_number);
        match missing_number {
            None => {
                for (round_1_difference, round_1_number) in round_1.iter() {
                    let round_2_difference = round_1_difference - *round_2_number;
                    round_2.insert(round_2_difference, (*round_1_number, *round_2_number));
                }
            }
            Some((a, b)) => return Some((*a, *b, *round_2_number)),
        }
    }

    None
}

#[cfg(test)]
mod tests {
    #[test]
    fn example() {
        const TARGET_SUM: i32 = 2020;
        let input_numbers = vec![1721, 979, 366, 299, 675, 1456];
        let solution = 979 * 366 * 675; // = 241861950
        let output = super::solve(input_numbers, TARGET_SUM);
        match output {
            Some((a, b, c)) => assert_eq!(solution, a * b * c),
            None => assert!(false),
        }
    }
}
