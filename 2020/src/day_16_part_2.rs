use itertools::Itertools;
use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

#[derive(Clone)]
struct Field {
    name: String,
    valid_range: HashSet<u32>,
}

pub(crate) fn process(input_filename: &str) {
    let file = File::open(input_filename).unwrap();
    let reader = BufReader::new(file);

    let range_regex = Regex::new(r"^(\w+\s*\w+): (\d+)-(\d+) or (\d+)-(\d+)$").unwrap();
    let mut is_ranges_mode_active = true;
    let mut is_my_ticket_mode_active = false;
    let mut is_nearby_tickets_mode_active = false;

    let mut field_descriptions: Vec<Field> = Vec::new();
    let mut my_ticket = Vec::new();
    let mut other_tickets: Vec<u32> = Vec::new();
    // rust lifetimes are weird so we can't have Vec<Vec<u32> as a list of columns
    // so we store it flat and do custom steps.

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
                    let name = capture.get(1).unwrap().as_str().into();

                    let range_1_start = u32::from_str(capture.get(2).unwrap().as_str()).unwrap();
                    let range_1_end = u32::from_str(capture.get(3).unwrap().as_str()).unwrap();
                    let range_2_start = u32::from_str(capture.get(4).unwrap().as_str()).unwrap();
                    let range_2_end = u32::from_str(capture.get(5).unwrap().as_str()).unwrap();
                    let mut valid_range = HashSet::new();
                    valid_range.extend(range_1_start..=range_1_end);
                    valid_range.extend(range_2_start..=range_2_end);

                    let field = Field { name, valid_range };
                    field_descriptions.push(field);
                }
            } else if is_my_ticket_mode_active {
                if line.contains(':') {
                    continue;
                }

                for field_string in line.split(',') {
                    let parsed_field = u32::from_str(field_string).unwrap();
                    my_ticket.push(parsed_field);
                }
            } else if is_nearby_tickets_mode_active {
                if line.contains(':') {
                    continue;
                }

                let mut is_ticket_valid = true;
                let mut ticket = Vec::new();
                for (field_string, description) in line.split(',').zip(field_descriptions.iter()) {
                    let parsed_field = u32::from_str(field_string).unwrap();

                    if description.valid_range.contains(&parsed_field) {
                        ticket.push(parsed_field);
                    } else {
                        is_ticket_valid = false;
                        break;
                    }
                }

                if is_ticket_valid {
                    other_tickets.extend(ticket);
                }
            } else {
                panic!(91610);
            }
        }
    }

    let step_size = 20;
    let target_prefix: String = String::from("departure");
    let field_descriptions: Vec<String> = solve(&field_descriptions, other_tickets, step_size);
    let mut product: u32 = 1;
    for (description, my_field) in field_descriptions.iter().zip(my_ticket.iter()) {
        if description.starts_with(&target_prefix) {
            product *= *my_field;
        }
    }
    println!(
        "Product of fields starting with \"{}\": {}",
        target_prefix, product
    );
}

fn solve(field_descriptions: &Vec<Field>, other_tickets: Vec<u32>, step_size: usize) -> Vec<String> {
    let mut possible_columns: Vec<Vec<usize>> = Vec::new();
    for description in field_descriptions.clone() {
        let mut possible_columns_for_rule: Vec<usize> = Vec::new();
        for skip in 0..step_size {
            let mut all_valid = true;
            for element in other_tickets.iter().skip(skip).step_by(step_size) {
                let is_valid = description.valid_range.contains(element);
                if !is_valid {
                    all_valid = false;
                    break;
                }
            }
            if all_valid {
                possible_columns_for_rule.push(skip);
            }
        }
        assert!(possible_columns_for_rule.len() > 0);
        possible_columns.push(possible_columns_for_rule);
    }

    possible_columns.sort_by(|a, b| a.len().partial_cmp(&b.len()).unwrap());
    let mut column_index_to_rule_index: HashMap<usize, usize> = HashMap::new();
    for _round in 0..100 {
        for (rule_index, fields) in possible_columns.iter().enumerate() {
            let unused_columns: Vec<&usize> = fields
                .iter()
                .filter(|field_index| !column_index_to_rule_index.contains_key(*field_index))
                .collect();

            if unused_columns.len() > 0 {
                let column_index = **unused_columns.get(0).unwrap();
                column_index_to_rule_index.insert(column_index, rule_index);
            }
        }
    }

    let mut rule_index_to_column_index: HashMap<usize, usize> = HashMap::new();
    for (k, v) in column_index_to_rule_index {
        rule_index_to_column_index.insert(v, k);
    }

    let mut result = Vec::new();
    for (_k, v) in rule_index_to_column_index.iter().sorted() {
        let field: Field = field_descriptions.get(*v).unwrap().clone();
        result.push(field.name);
    }

    result
}

fn _solve2(
    field_descriptions: Vec<Field>,
    other_tickets: Vec<u32>,
    step_size: usize,
) -> Vec<String> {
    let mut possible_fields = Vec::new();
    for skip in 0..step_size {
        let mut possible_fields_for_column = Vec::new();
        for (index, description) in field_descriptions.iter().enumerate() {
            let mut all_valid = true;
            for element in other_tickets.iter().skip(skip).step_by(step_size) {
                let is_valid = description.valid_range.contains(element);
                if !is_valid {
                    all_valid = false;
                    break;
                }
            }
            if all_valid {
                possible_fields_for_column.push(index);
            }
        }
        assert!(possible_fields_for_column.len() > 0);
        possible_fields.push(possible_fields_for_column);
    }

    let mut field_index_to_assigned_index: HashMap<usize, usize> = HashMap::new();
    for _round in 0..field_descriptions.len() {
        for (index, fields) in possible_fields.iter().enumerate() {
            let unused_fields: Vec<&usize> = fields
                .iter()
                .filter(|field_index| !field_index_to_assigned_index.contains_key(*field_index))
                .collect();

            if unused_fields.len() == 1 {
                let field_index = **unused_fields.get(0).unwrap();
                field_index_to_assigned_index.insert(field_index, index);
            }
        }
    }

    let mut assigned_index_to_field_index: HashMap<usize, usize> = HashMap::new();
    for (k, v) in field_index_to_assigned_index {
        assigned_index_to_field_index.insert(v, k);
    }

    let mut result = Vec::new();
    for (_k, v) in assigned_index_to_field_index.iter().sorted() {
        let field: Field = field_descriptions.get(*v).unwrap().clone();
        result.push(field.name);
    }

    result
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    #[test]
    fn example() {
        let field_descriptions: Vec<super::Field> = vec![
            super::Field {
                name: String::from("class"),
                valid_range: {
                    let mut set = HashSet::new();
                    set.extend(0..=1);
                    set.extend(4..=19);
                    set
                },
            },
            super::Field {
                name: String::from("row"),
                valid_range: {
                    let mut set = HashSet::new();
                    set.extend(0..=5);
                    set.extend(8..=19);
                    set
                },
            },
            super::Field {
                name: String::from("seat"),
                valid_range: {
                    let mut set = HashSet::new();
                    set.extend(0..=13);
                    set.extend(16..=19);
                    set
                },
            },
        ];
        let other_tickets: Vec<u32> = vec![3, 9, 18, 15, 1, 5, 5, 14, 9];
        let step_size = 3;

        let solution = vec![
            String::from("row"),
            String::from("class"),
            String::from("seat"),
        ];
        let output: Vec<String> = super::solve(&field_descriptions, other_tickets, step_size);
        assert_eq!(solution, output);
    }
}
