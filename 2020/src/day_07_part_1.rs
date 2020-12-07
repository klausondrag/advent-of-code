use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub(crate) fn process(input_filename: &str) {
    let file = File::open(input_filename).unwrap();
    let reader = BufReader::new(file);

    const SPLIT_KEY: &str = "contain";
    let value_regex = Regex::new(r"\d+ (\w+ \w+) \w+").unwrap();
    let mut rules = HashMap::new();
    for line in reader.lines() {
        let line = line.unwrap();

        let key_value: Vec<&str> = line.split(SPLIT_KEY).collect();
        let mut key = match key_value.get(0) {
            None => panic!(""),
            Some(k) => k.to_string(),
        };
        key.truncate(key.len() - 6); // " bags "
        let values = match key_value.get(1) {
            None => panic!(""),
            Some(v) => *v,
        };
        let value: Vec<String> = value_regex
            .captures_iter(values)
            .map(|group| group[1].parse().unwrap())
            .collect();
        rules.insert(key, value);
    }

    for key in vec![
        String::from("bright indigo"),
        String::from("light gold"),
        String::from("muted tan"),
    ] {
        println!("{}", key);
        for value in rules.get(&*key).unwrap() {
            println!("{}", value);
        }
        println!();
    }
    let n_possible_bags = solve(&rules);
    println!("Found n possible outer bags: {}", n_possible_bags);
}

fn solve(rules: &HashMap<String, Vec<String>>) -> i32 {
    let mut cache: HashMap<&String, bool> = HashMap::new();
    let mut sum = 0;
    for key in rules.keys() {
        if *key == String::from("shiny gold") {
            continue;
        }

        let is_okay = solve_key(key, rules, &mut cache);
        if is_okay {
            sum += 1;
        }
    }
    sum
}

fn solve_key<'a>(
    key: &'a String,
    rules: &'a HashMap<String, Vec<String>>,
    cache: &mut HashMap<&'a String, bool>,
) -> bool {
    if cache.contains_key(key) {
        match cache.get(key) {
            None => false, // should never happen
            Some(v) => *v,
        }
    } else {
        let values = match rules.get(key) {
            None => panic!("ERROR key not found: {}", key),
            Some(v) => v,
        };
        for value in values.iter() {
            if *value == String::from("shiny gold") {
                cache.insert(key, true);
                return true;
            }

            let b = solve_key(value, rules, cache);
            cache.insert(key, b);
            if b {
                return true;
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    #[test]
    fn example() {
        let mut input_rules: HashMap<String, Vec<String>> = HashMap::new();
        input_rules.insert(
            String::from("light red"),
            vec![String::from("bright white"), String::from("muted yellow")],
        );
        input_rules.insert(
            String::from("dark orange"),
            vec![String::from("bright white"), String::from("muted yellow")],
        );
        input_rules.insert(
            String::from("bright white"),
            vec![String::from("shiny gold")],
        );
        input_rules.insert(
            String::from("muted yellow"),
            vec![String::from("shiny gold"), String::from("faded blue")],
        );
        input_rules.insert(
            String::from("shiny gold"),
            vec![String::from("dark olive"), String::from("vibrant plum")],
        );
        input_rules.insert(
            String::from("dark olive"),
            vec![String::from("faded blue"), String::from("dotted black")],
        );
        input_rules.insert(
            String::from("vibrant plum"),
            vec![String::from("faded blue"), String::from("dotted black")],
        );
        input_rules.insert(String::from("faded blue"), vec![]);
        input_rules.insert(String::from("dotted black"), vec![]);
        let solution = 4;
        let output = super::solve(&input_rules);
        assert_eq!(solution, output);
    }
}
