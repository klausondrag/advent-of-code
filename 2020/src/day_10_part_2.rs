use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub(crate) fn process(input_filename: &str) {
    let file = File::open(input_filename).unwrap();
    let reader = BufReader::new(file);

    let mut inputs: Vec<usize> = Vec::new();
    for line in reader.lines() {
        let line = line.unwrap();
        let input_number = line.parse::<usize>().unwrap();
        inputs.push(input_number);
    }

    let n_paths = solve(inputs);
    println!("Found n possible paths: {}", n_paths);
}

fn solve(mut adapters: Vec<usize>) -> u64 {
    adapters.sort();
    let max = *adapters.get(adapters.len() - 1).unwrap();
    adapters.insert(0, 0);
    adapters.push(max + 3);

    let mut paths_counter: HashMap<usize, u64> = HashMap::new();
    paths_counter.insert(0, 1);

    for (index, adapter) in adapters.iter().enumerate().skip(1) {
        let mut paths: u64 = 0;
        for i in (0..index).rev() {
            let other_adapter = *adapters.get(i).unwrap();
            if (adapter - other_adapter) > 3 {
                break;
            }

            paths += paths_counter.get(&i).unwrap_or(&0);
        }

        paths_counter.insert(index, paths);
    }

    *paths_counter.get(&(adapters.len() - 1)).unwrap()
}

#[cfg(test)]
mod tests {
    #[test]
    fn examples() {
        let inputs: Vec<Vec<usize>> = vec![
            vec![16, 10, 15, 5, 1, 11, 7, 19, 6, 12, 4],
            vec![
                28, 33, 18, 42, 31, 14, 46, 20, 48, 47, 24, 23, 49, 45, 19, 38, 39, 11, 1, 32, 25,
                35, 8, 17, 7, 9, 4, 2, 34, 10, 3,
            ],
        ];
        let solutions = vec![8, 19208];
        let outputs: Vec<u64> = inputs.iter().map(|v| super::solve(v.clone())).collect();
        assert_eq!(solutions, outputs);
    }
}
