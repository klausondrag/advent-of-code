use regex::Regex;
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::iter::FromIterator;
use std::str::FromStr;

pub(crate) fn process(input_filename: &str) {
    let file = File::open(input_filename).unwrap();
    let reader = BufReader::new(file);

    let range_regex = Regex::new(r"^.*: (\d+)-(\d+) or (\d+)-(\d+)$").unwrap();
    let mut is_ranges_mode_active = true;
    let mut is_my_ticket_mode_active = false;
    let mut is_nearby_tickets_mode_active = false;
    let mut x = Vec::new();
    let mut nearby_tickets = Vec::new();
    for line in reader.lines() {
        let line = line.unwrap();
        if line == "" {
            if is_ranges_mode_active {
                is_ranges_mode_active = false;
                is_my_ticket_mode_active = true;
                is_nearby_tickets_mode_active = false;
            } else if is_my_ticket_mode_active {
                is_ranges_mode_active = false;
                is_my_ticket_mode_active = false;
                is_nearby_tickets_mode_active = true;
            } else {
                panic!(91610);
            }
        } else {
            if is_ranges_mode_active {
                for capture in range_regex.captures_iter(&*line) {
                    let range_1_start = u32::from_str(capture.get(1).unwrap().as_str()).unwrap();
                    let range_1_end = u32::from_str(capture.get(2).unwrap().as_str()).unwrap();
                    let range_2_start = u32::from_str(capture.get(3).unwrap().as_str()).unwrap();
                    let range_2_end = u32::from_str(capture.get(4).unwrap().as_str()).unwrap();
                    x.push(range_1_start..=range_1_end);
                    x.push(range_2_start..=range_2_end);
                }
            } else if is_my_ticket_mode_active {
                continue;
            } else if is_nearby_tickets_mode_active {
                if line.contains(':') {
                    continue;
                }

                let mut ticket = Vec::new();
                for field_string in line.split(',') {
                    let parsed_field = u32::from_str(field_string).unwrap();
                    ticket.push(parsed_field);
                }
                nearby_tickets.push(ticket);
            } else {
                panic!(91610);
            }
        }
    }

    let y: Vec<u32> = x
        .iter()
        .flat_map(|r| r.clone().collect::<Vec<u32>>())
        .collect();
    let valid_ranges: HashSet<&u32> = HashSet::from_iter(y.iter());
    let valid_fields = solve(valid_ranges, nearby_tickets);
    println!(
        "Found n invalid fields: {}",
        valid_fields.iter().sum::<u32>()
    );
}

fn solve(valid_ranges: HashSet<&u32>, nearby_tickets: Vec<Vec<u32>>) -> Vec<u32> {
    let x: Vec<u32> = nearby_tickets
        .iter()
        .flatten()
        .filter(|nt| !valid_ranges.contains(*nt))
        .map(|v| v.clone())
        .collect();

    x.clone()
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;
    use std::iter::FromIterator;

    #[test]
    fn example() {
        let x = vec![1..=3, 5..=7, 6..=11, 33..=44, 13..=40, 45..=50];
        let y: Vec<u32> = x
            .iter()
            .flat_map(|r| r.clone().collect::<Vec<u32>>())
            .collect();
        let valid_ranges: HashSet<&u32> = HashSet::from_iter(y.iter());
        let nearby_tickets: Vec<Vec<u32>> = vec![
            vec![7, 3, 47],
            vec![40, 4, 50],
            vec![55, 2, 20],
            vec![38, 6, 12],
        ];
        let solution: Vec<u32> = vec![4, 55, 12]; // sum = 71
        let output: Vec<u32> = super::solve(valid_ranges, nearby_tickets);
        assert_eq!(solution, output);
    }
}
