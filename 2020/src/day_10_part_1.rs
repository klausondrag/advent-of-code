use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub(crate) fn process(input_filename: &str) {
    let file = File::open(input_filename).unwrap();
    let reader = BufReader::new(file);

    let mut inputs: Vec<u32> = Vec::new();
    for line in reader.lines() {
        let line = line.unwrap();
        let input_number = line.parse::<u32>().unwrap();
        inputs.push(input_number);
    }

    let (one_differences, _two_differences, three_differences) = solve(inputs);
    println!(
        "{} 1-jolt differences * {} 3-jolt differences = {}",
        one_differences,
        three_differences,
        one_differences * three_differences
    );
}

fn solve(mut adapters: Vec<u32>) -> (u32, u32, u32) {
    adapters.sort();
    let max = *adapters.get(adapters.len() - 1).unwrap();
    adapters.insert(0, 0);
    adapters.push(max + 3);

    let mut counter: HashMap<u32, u32> = HashMap::new();
    for i in 1..4 {
        counter.insert(i, 0);
    }

    for (previous_adapter, current_adapter) in adapters.iter().zip(adapters.iter().skip(1)) {
        let difference = *current_adapter - *previous_adapter;
        counter.insert(difference, counter.get(&difference).unwrap() + 1);
    }

    (
        *counter.get(&1).unwrap(),
        *counter.get(&2).unwrap(),
        *counter.get(&3).unwrap(),
    )
}

#[cfg(test)]
mod tests {
    #[test]
    fn examples() {
        let inputs: Vec<Vec<u32>> = vec![
            vec![16, 10, 15, 5, 1, 11, 7, 19, 6, 12, 4],
            vec![
                28, 33, 18, 42, 31, 14, 46, 20, 48, 47, 24, 23, 49, 45, 19, 38, 39, 11, 1, 32, 25,
                35, 8, 17, 7, 9, 4, 2, 34, 10, 3,
            ],
        ];
        let solutions = vec![7 * 5, 22 * 10]; // = 35, 220
        let outputs: Vec<(u32, u32, u32)> =
            inputs.iter().map(|v| super::solve(v.clone())).collect();
        for (s, (o1, _o2, o3)) in solutions.iter().zip(outputs.iter()) {
            assert_eq!(*s, *o1 * *o3);
        }
    }
}
