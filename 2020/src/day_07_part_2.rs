use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

struct Bag {
    n: i32,
    look: String,
}

pub(crate) fn process(input_filename: &str) {
    let file = File::open(input_filename).unwrap();
    let reader = BufReader::new(file);

    const SPLIT_KEY: &str = "contain";
    let value_regex = Regex::new(r"(\d+) (\w+ \w+) \w+").unwrap();
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
        let value: Vec<Bag> = value_regex
            .captures_iter(values)
            .map(|group| Bag {
                n: i32::from_str(&group[1]).unwrap(),
                look: group[2].parse().unwrap(),
            })
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
            println!("{}", value.look);
        }
        println!();
    }
    let n_possible_bags = solve(&rules);
    println!("Found n possible outer bags: {}", n_possible_bags);
}

fn solve(rules: &HashMap<String, Vec<Bag>>) -> i32 {
    let mut cache: HashMap<&String, i32> = HashMap::new();
    solve_key(&String::from("shiny gold"), rules, &mut cache)
}

fn solve_key<'a>(
    key: &'a String,
    rules: &'a HashMap<String, Vec<Bag>>,
    cache: &mut HashMap<&'a String, i32>,
) -> i32 {
    if cache.contains_key(key) {
        match cache.get(key) {
            None => -99999, // should never happen
            Some(v) => *v,
        }
    } else {
        let values = match rules.get(key) {
            None => return 0,
            Some(v) => v,
        };
        let mut n = 0;
        for value in values.iter() {
            // +1 because of the bag itself
            n += value.n * (solve_key(&value.look, rules, cache) + 1);
        }
        cache.insert(key, n);
        n
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    #[test]
    fn example1() {
        let mut input_rules: HashMap<String, Vec<super::Bag>> = HashMap::new();
        input_rules.insert(
            String::from("light red"),
            vec![
                super::Bag {
                    n: 1,
                    look: String::from("bright white"),
                },
                super::Bag {
                    n: 2,
                    look: String::from("muted yellow"),
                },
            ],
        );
        input_rules.insert(
            String::from("dark orange"),
            vec![
                super::Bag {
                    n: 3,
                    look: String::from("bright white"),
                },
                super::Bag {
                    n: 4,
                    look: String::from("muted yellow"),
                },
            ],
        );
        input_rules.insert(
            String::from("bright white"),
            vec![super::Bag {
                n: 1,
                look: String::from("shiny gold"),
            }],
        );
        input_rules.insert(
            String::from("muted yellow"),
            vec![
                super::Bag {
                    n: 2,
                    look: String::from("shiny gold"),
                },
                super::Bag {
                    n: 9,
                    look: String::from("faded blue"),
                },
            ],
        );
        input_rules.insert(
            String::from("shiny gold"),
            vec![
                super::Bag {
                    n: 1,
                    look: String::from("dark olive"),
                },
                super::Bag {
                    n: 2,
                    look: String::from("vibrant plum"),
                },
            ],
        );
        input_rules.insert(
            String::from("dark olive"),
            vec![
                super::Bag {
                    n: 3,
                    look: String::from("faded blue"),
                },
                super::Bag {
                    n: 4,
                    look: String::from("dotted black"),
                },
            ],
        );
        input_rules.insert(
            String::from("vibrant plum"),
            vec![
                super::Bag {
                    n: 5,
                    look: String::from("faded blue"),
                },
                super::Bag {
                    n: 6,
                    look: String::from("dotted black"),
                },
            ],
        );
        input_rules.insert(String::from("faded blue"), vec![]);
        input_rules.insert(String::from("dotted black"), vec![]);
        let solution = 1 + 1 * 7 + 2 + 2 * 11; // = 32
        let output = super::solve(&input_rules);
        assert_eq!(solution, output);
    }

    #[test]
    fn example2() {
        let mut input_rules: HashMap<String, Vec<super::Bag>> = HashMap::new();
        input_rules.insert(
            String::from("shiny gold"),
            vec![super::Bag {
                n: 2,
                look: String::from("dark red"),
            }],
        );
        input_rules.insert(
            String::from("dark red"),
            vec![super::Bag {
                n: 2,
                look: String::from("dark orange"),
            }],
        );
        input_rules.insert(
            String::from("dark orange"),
            vec![super::Bag {
                n: 2,
                look: String::from("dark yellow"),
            }],
        );
        input_rules.insert(
            String::from("dark yellow"),
            vec![super::Bag {
                n: 2,
                look: String::from("dark green"),
            }],
        );
        input_rules.insert(
            String::from("dark green"),
            vec![super::Bag {
                n: 2,
                look: String::from("dark blue"),
            }],
        );
        input_rules.insert(
            String::from("dark blue"),
            vec![super::Bag {
                n: 2,
                look: String::from("dark violet"),
            }],
        );
        input_rules.insert(String::from("dark violet"), vec![]);
        let solution = 126;
        let output = super::solve(&input_rules);
        assert_eq!(solution, output);
    }
}
