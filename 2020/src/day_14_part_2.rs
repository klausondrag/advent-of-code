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
    let write_regex = Regex::new(r"mem\[(\d+)\] = (\d+)").unwrap();

    let mut memory: HashMap<usize, u64> = HashMap::new();
    let mut mask: String = String::from("");

    for input in inputs {
        if input.contains('[') {
            // write
            for capture in write_regex.captures_iter(&*input) {
                let address_to_write = usize::from_str(capture.get(1).unwrap().as_str()).unwrap();
                let value = u64::from_str(capture.get(2).unwrap().as_str()).unwrap();
                let all_addresses = floating_addresses(mask.clone(), address_to_write);
                for address in all_addresses {
                    memory.insert(address, value);
                }
            }
        } else {
            // new mask
            mask = input.chars().skip(7).collect();
        }
    }

    let mut sum_of_memory = 0;
    for value in memory.values() {
        sum_of_memory += *value;
    }

    sum_of_memory
}

fn floating_addresses(mask: String, address: usize) -> Vec<usize> {
    let mut string_addresses: Vec<String> = vec![String::from("")];
    let address_string = format!("{:036b}", address);
    for (address_bit, mask_bit) in address_string.chars().zip(mask.chars()) {
        string_addresses = match (address_bit, mask_bit) {
            ('0', '0') => {
                let mut extended_addresses = Vec::new();
                for old_address in string_addresses {
                    let mut extended_address = old_address.clone();
                    extended_address.push('0');
                    extended_addresses.push(extended_address);
                }
                extended_addresses
            }
            ('1', '0') | ('1', '1') | (_, '1') => {
                let mut extended_addresses = Vec::new();
                for old_address in string_addresses {
                    let mut extended_address = old_address.clone();
                    extended_address.push('1');
                    extended_addresses.push(extended_address);
                }
                extended_addresses
            }
            (_, 'X') => {
                let mut extended_addresses = Vec::new();
                for old_address in string_addresses {
                    let mut extended_address = old_address.clone();
                    extended_address.push('0');
                    extended_addresses.push(extended_address);

                    extended_address = old_address.clone();
                    extended_address.push('1');
                    extended_addresses.push(extended_address);
                }
                extended_addresses
            }
            _ => panic!(91420),
        }
    }

    string_addresses
        .iter()
        .map(|s| usize::from_str_radix(&*s.clone(), 2).unwrap())
        .collect()
}

#[cfg(test)]
mod tests {
    #[test]
    fn examples() {
        let input = vec![
            String::from("mask = 000000000000000000000000000000X1001X"),
            String::from("mem[42] = 100"),
            String::from("mask = 00000000000000000000000000000000X0XX"),
            String::from("mem[26] = 1"),
        ];
        let solution: u64 = 208;
        let output = super::solve(input);
        assert_eq!(solution, output);
    }
}
