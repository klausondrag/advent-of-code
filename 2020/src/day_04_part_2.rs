use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::process::exit;
use std::str::FromStr;

#[derive(Clone)]
struct Passport {
    byr: Option<i32>,
    iyr: Option<i32>,
    eyr: Option<i32>,
    hgt_value: Option<i32>,
    hgt_unit: Option<String>,
    hcl: Option<String>,
    ecl: Option<String>,
    pid: Option<String>,
    cid: Option<String>,
}

pub(crate) fn process(input_filename: &str) {
    let file = File::open(input_filename).unwrap();
    let reader = BufReader::new(file);
    let hgt_regex = Regex::new(r"^(\d+)(\w+)$").unwrap();

    let mut n_valid_passports = 0;
    let mut passport = Passport {
        byr: None,
        iyr: None,
        eyr: None,
        hgt_value: None,
        hgt_unit: None,
        hcl: None,
        ecl: None,
        pid: None,
        cid: None,
    };
    for line in reader.lines() {
        let line = line.unwrap();
        if line == "" {
            let is_valid = solve(passport.clone());
            if is_valid {
                n_valid_passports += 1;
            }
            passport = Passport {
                byr: None,
                iyr: None,
                eyr: None,
                hgt_value: None,
                hgt_unit: None,
                hcl: None,
                ecl: None,
                pid: None,
                cid: None,
            };
        } else {
            for field in line.split(" ") {
                let pair: Vec<&str> = field.split(":").collect();
                if pair.len() != 2 {
                    println!("Error: Invalid line `{}`", line);
                    // error specification: 9XXYZ for the Z-th error of day XX part Y
                    exit(90420);
                }
                let key = pair[0];
                let value = pair[1];
                match key {
                    "byr" => passport.byr = parse_int(value),
                    "iyr" => passport.iyr = parse_int(value),
                    "eyr" => passport.eyr = parse_int(value),
                    "hgt" => {
                        for group in hgt_regex.captures_iter(value) {
                            passport.hgt_value = parse_int(&*group[1].to_string());
                            passport.hgt_unit = Some(group[2].to_string());
                        }
                    }
                    "hcl" => passport.hcl = Some(value.to_string()),
                    "ecl" => passport.ecl = Some(value.to_string()),
                    "pid" => passport.pid = Some(value.to_string()),
                    "cid" => passport.cid = Some(value.to_string()),
                    _ => {
                        println!("Error: Invalid line `{}`", line);
                        // error specification: 9XXYZ for the Z-th error of day XX part Y
                        exit(90421);
                    }
                }
            }
        }
    }
    // Parse one more time for the last passport in case there was no empty line.
    // Should the last passport already have been parsed,
    // it will have been initialized with None values.
    // This means it will be not valid anyways.
    let is_valid = solve(passport.clone());
    if is_valid {
        n_valid_passports += 1;
    }

    println!("Found n valid passports: {}", n_valid_passports);
}

fn parse_int(value: &str) -> Option<i32> {
    match i32::from_str(value) {
        Ok(n) => Some(n),
        Err(_e) => None,
    }
}

fn solve(input: Passport) -> bool {
    let hcl_regex: Regex = Regex::new(r"^#[a-fA-F0-9]{6}$").unwrap();
    match input {
        Passport {
            byr: Some(1920..=2002),
            iyr: Some(2010..=2020),
            eyr: Some(2020..=2030),
            hgt_value: Some(ref hgt_value),
            hgt_unit: Some(ref hgt_unit),
            hcl: Some(ref hcl),
            ecl: Some(ref ecl),
            pid: Some(ref pid),
            cid: _,
        } => {
            (match &hgt_unit[..] {
                "cm" => match *hgt_value {
                    150..=193 => true,
                    _ => false,
                },
                "in" => match *hgt_value {
                    59..=76 => true,
                    _ => false,
                },
                _ => false,
            }) && hcl_regex.is_match(hcl)
                && (match &ecl[..] {
                    "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth" => true,
                    _ => false,
                })
                && (pid.len() == 9)
        }
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn example() {
        let inputs = vec![
            super::Passport {
                eyr: Some(1972),
                cid: Some(String::from("100")),
                hcl: Some(String::from("#18171d")),
                ecl: Some(String::from("amb")),
                hgt_value: Some(170),
                hgt_unit: Some(String::from("")),
                pid: Some(String::from("186cm")),
                iyr: Some(2018),
                byr: Some(1926),
            },
            super::Passport {
                iyr: Some(2019),
                hcl: Some(String::from("#602927")),
                eyr: Some(1967),
                hgt_value: Some(170),
                hgt_unit: Some(String::from("cm")),
                ecl: Some(String::from("grn")),
                pid: Some(String::from("012533040")),
                byr: Some(1946),
                cid: None,
            },
            super::Passport {
                hcl: Some(String::from("dab227")),
                iyr: Some(2012),
                ecl: Some(String::from("brn")),
                hgt_value: Some(182),
                hgt_unit: Some(String::from("cm")),
                pid: Some(String::from("021572410")),
                eyr: Some(2020),
                byr: Some(1992),
                cid: Some(String::from("277")),
            },
            super::Passport {
                hgt_value: Some(59),
                hgt_unit: Some(String::from("cm")),
                ecl: Some(String::from("zzz")),
                eyr: Some(2038),
                hcl: Some(String::from("74454a")),
                iyr: Some(2023),
                pid: Some(String::from("3556412378")),
                byr: Some(2007),
                cid: None,
            },
            super::Passport {
                pid: Some(String::from("087499704")),
                hgt_value: Some(74),
                hgt_unit: Some(String::from("in")),
                ecl: Some(String::from("grn")),
                iyr: Some(2012),
                eyr: Some(2030),
                byr: Some(1980),
                hcl: Some(String::from("#623a2f")),

                cid: None,
            },
            super::Passport {
                eyr: Some(2029),
                ecl: Some(String::from("blu")),
                cid: Some(String::from("129")),
                byr: Some(1989),
                iyr: Some(2014),
                pid: Some(String::from("896056539")),
                hcl: Some(String::from("#a97842")),
                hgt_value: Some(165),
                hgt_unit: Some(String::from("cm")),
            },
            super::Passport {
                hcl: Some(String::from("#888785")),
                hgt_value: Some(164),
                hgt_unit: Some(String::from("cm")),
                byr: Some(2001),
                iyr: Some(2015),
                cid: Some(String::from("88")),
                pid: Some(String::from("545766238")),
                ecl: Some(String::from("hzl")),
                eyr: Some(2022),
            },
            super::Passport {
                iyr: Some(2010),
                hgt_value: Some(158),
                hgt_unit: Some(String::from("cm")),
                hcl: Some(String::from("#b6652a")),
                ecl: Some(String::from("blu")),
                byr: Some(1944),
                eyr: Some(2021),
                pid: Some(String::from("093154719")),
                cid: None,
            },
        ];
        let solutions = vec![false, false, false, false, true, true, true, true]; // = 4
        let outputs: Vec<bool> = inputs.into_iter().map(super::solve).collect();
        assert_eq!(solutions, outputs);
    }
}
