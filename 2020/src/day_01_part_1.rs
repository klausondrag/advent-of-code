use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::process::exit;

pub(crate) fn process(input_filename: &str) {
    let file = File::open(input_filename).unwrap();
    let reader = BufReader::new(file);

    const TARGET_SUM: i32 = 2020;
    let mut cache: HashMap<i32, i32> = HashMap::new();
    for line in reader.lines() {
        let line = line.unwrap();
        let input_number = line.parse::<i32>().unwrap();

        let missing_number = cache.get(&input_number);
        match missing_number {
            Some(x) => {
                let result = x * input_number;
                println!(
                    "Found Numbers.\n{} + {} = {}\n{} * {} = {}",
                    x, input_number, TARGET_SUM, x, input_number, result
                );
                return;
            }
            None => {
                let difference = TARGET_SUM - input_number;
                cache.insert(difference, input_number);
            },
        }
    }

    println!("Error: No numbers adding up to {} found!", TARGET_SUM);
    exit(0x011);
}

#[cfg(test)]
mod tests {
    #[test]
    fn examples() {
        let inputs = vec![12, 14, 1969, 100756];
        let solutions = vec![2, 2, 654, 33583];
        let outputs: Vec<i32> = inputs.into_iter().map(super::solve).collect();
        assert_eq!(solutions, outputs);
    }
}
