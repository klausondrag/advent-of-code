mod day_01_part_1;
mod day_01_part_2;
mod day_02_part_1;
mod day_02_part_2;
mod day_03_part_1;
mod day_03_part_2;
mod day_04_part_1;
mod day_04_part_2;
mod day_05_part_1;
mod day_05_part_2;

use std::collections::HashMap;
use std::path::Path;
use std::{env, process};

fn main() {
    let args: Vec<String> = env::args().collect();
    let day = &args[1];
    let part = &args[2];

    let input_filename = &format!("./inputs/{}.txt", day);
    let input_file = Path::new(input_filename);
    if !input_file.exists() {
        println!("Input file {} does not exist. Aborting.", input_filename);
        process::exit(1);
    }

    let mut programs: HashMap<String, fn(&str) -> ()> = HashMap::new();
    programs.insert(String::from("01-1"), day_01_part_1::process);
    programs.insert(String::from("01-2"), day_01_part_2::process);
    programs.insert(String::from("02-1"), day_02_part_1::process);
    programs.insert(String::from("02-2"), day_02_part_2::process);
    programs.insert(String::from("03-1"), day_03_part_1::process);
    programs.insert(String::from("03-2"), day_03_part_2::process);
    programs.insert(String::from("04-1"), day_04_part_1::process);
    programs.insert(String::from("04-2"), day_04_part_2::process);
    programs.insert(String::from("05-1"), day_05_part_1::process);
    programs.insert(String::from("05-2"), day_05_part_2::process);

    let key = day.to_owned() + "-" + part;
    let program = programs.get(&key);
    match program {
        Some(program) => program(input_filename),
        None => {
            println!("Not yet implemented.");
            process::exit(2);
        }
    };
}
