use std::fs::File;
use std::io::{BufReader, BufRead};

pub(crate) fn process(input_filename: &str) {
    let file = File::open(input_filename).unwrap();
    let reader = BufReader::new(file);

    let mut sum = 0;
    for line in reader.lines() {
        let line = line.unwrap();
        let puzzle_input = line.parse::<i32>().unwrap();
        sum += solve(puzzle_input);
    }

    println!("Total fuel requirement: {}", sum);
}

fn solve(input: i32) -> i32 {
    (input / 3) - 2
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
