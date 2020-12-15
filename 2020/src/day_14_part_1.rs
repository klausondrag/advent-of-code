use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

pub(crate) fn process(input_filename: &str) {
    let file = File::open(input_filename).unwrap();
    let reader = BufReader::new(file);

    let mut input = Vec::new();
    for line in reader.lines() {
        let line = line.unwrap();
        input.push(line);
    }

    let sum_of_memory = solve(input);
    println!("Sum of memory: {}", sum_of_memory);
}

fn solve(inputs: Vec<String>) -> u64 {
    let mut zero_mask: u64 = u64::max_value();
    let mut one_mask: u64 = 0;
    let mut memory: HashMap<usize, u64> = HashMap::new();
    let offset = 35;

    let write_regex = Regex::new(r"mem\[(\d+)\] = (\d+)").unwrap();
    for input in inputs {
        if input.contains('[') {
            // write
            for capture in write_regex.captures_iter(&*input) {
                let address = usize::from_str(capture.get(1).unwrap().as_str()).unwrap();
                let mut value = u64::from_str(capture.get(2).unwrap().as_str()).unwrap();
                value &= zero_mask;
                value |= one_mask;
                memory.insert(address, value);
            }
        } else {
            // new mask
            zero_mask = u64::max_value();
            one_mask = 0;
            for (index, bit) in input.chars().skip(7).enumerate() {
                match bit {
                    '0' => zero_mask &= !(1 << (offset - index)),
                    '1' => one_mask |= 1 << (offset - index),
                    'X' => {}
                    _ => panic!(91410),
                }
            }
        }
    }

    let mut sum_of_memory = 0;
    for value in memory.values() {
        sum_of_memory += *value;
    }

    sum_of_memory
}

#[cfg(test)]
mod tests {
    #[test]
    fn examples() {
        let input = vec![
            String::from("mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X"),
            String::from("mem[8] = 11"),
            String::from("mem[7] = 101"),
            String::from("mem[8] = 0"),
        ];
        let solution: u64 = 165;
        let output = super::solve(input);
        assert_eq!(solution, output);
    }
}
