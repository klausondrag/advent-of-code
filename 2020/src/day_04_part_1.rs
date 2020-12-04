use std::fs::File;
use std::io::{BufRead, BufReader};
use std::process::exit;

#[derive(Clone)]
struct Passport {
    byr: Option<String>,
    iyr: Option<String>,
    eyr: Option<String>,
    hgt: Option<String>,
    hcl: Option<String>,
    ecl: Option<String>,
    pid: Option<String>,
    cid: Option<String>,
}

pub(crate) fn process(input_filename: &str) {
    let file = File::open(input_filename).unwrap();
    let reader = BufReader::new(file);

    let mut n_valid_passports = 0;
    let mut passport = Passport {
        byr: None,
        iyr: None,
        eyr: None,
        hgt: None,
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
                hgt: None,
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
                    exit(90410);
                }
                let key = pair[0];
                let value = pair[1].parse().unwrap();
                match key {
                    "byr" => passport.byr = Some(value),
                    "iyr" => passport.iyr = Some(value),
                    "eyr" => passport.eyr = Some(value),
                    "hgt" => passport.hgt = Some(value),
                    "hcl" => passport.hcl = Some(value),
                    "ecl" => passport.ecl = Some(value),
                    "pid" => passport.pid = Some(value),
                    "cid" => passport.cid = Some(value),
                    _ => {
                        println!("Error: Invalid line `{}`", line);
                        // error specification: 9XXYZ for the Z-th error of day XX part Y
                        exit(90411);
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

fn solve(input: Passport) -> bool {
    match input {
        Passport {
            byr: Some(_a),
            iyr: Some(_b),
            eyr: Some(_c),
            hgt: Some(_d),
            hcl: Some(_e),
            ecl: Some(_f),
            pid: Some(_g),
            cid: _,
        } => true,
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn example() {
        let inputs = vec![
            super::Passport {
                ecl: Some(String::from("gry")),
                pid: Some(String::from("860033327")),
                eyr: Some(String::from("2020")),
                hcl: Some(String::from("#fffffd")),
                byr: Some(String::from("1937")),
                iyr: Some(String::from("2017")),
                cid: Some(String::from("147")),
                hgt: Some(String::from("183cm")),
            },
            super::Passport {
                iyr: Some(String::from("2013")),
                ecl: Some(String::from("amb")),
                cid: Some(String::from("350")),
                eyr: Some(String::from("2023")),
                pid: Some(String::from("028048884")),
                hcl: Some(String::from("#cfa07d")),
                byr: Some(String::from("1929")),
                hgt: None,
            },
            super::Passport {
                hcl: Some(String::from("#ae17e1")),
                iyr: Some(String::from("2013")),
                eyr: Some(String::from("2024")),
                ecl: Some(String::from("brn")),
                pid: Some(String::from("760753108")),
                byr: Some(String::from("1931")),
                hgt: Some(String::from("179cm")),
                cid: None,
            },
            super::Passport {
                byr: None,
                hcl: Some(String::from("#cfa07d")),
                eyr: Some(String::from("2025")),
                pid: Some(String::from("166559648")),
                iyr: Some(String::from("2011")),
                ecl: Some(String::from("brn")),
                hgt: Some(String::from("59in")),
                cid: None,
            },
        ];
        let solutions = vec![true, false, true, false]; // = 2
        let outputs: Vec<bool> = inputs.into_iter().map(super::solve).collect();
        assert_eq!(solutions, outputs);
    }
}
