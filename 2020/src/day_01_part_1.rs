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
        Some((a, b)) => {
            let result = a * b;
            println!(
                "Found Numbers.\n{} + {} = {}\n{} * {} = {}",
                a, b, TARGET_SUM, a, b, result
            );
        }
        None => {
            println!("Error: No numbers adding up to {} found!", TARGET_SUM);
            // error specification: 9XXYZ for the Z-th error of day XX part Y
            exit(90110);
        }
    }
}

fn solve(input_numbers: Vec<i32>, target_sum: i32) -> Option<(i32, i32)> {
    let mut cache: HashMap<i32, i32> = HashMap::new();
    for input_number in input_numbers {
        let missing_number = cache.get(&input_number);
        match missing_number {
            None => {
                let difference = target_sum - input_number;
                cache.insert(difference, input_number);
            }
            Some(missing_number) => return Some((*missing_number, input_number)),
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
        let solution = 1721 * 299; // = 514579
        let output = super::solve(input_numbers, TARGET_SUM);
        match output {
            Some((a, b)) => assert_eq!(solution, a * b),
            None => assert!(false),
        }
    }
}
