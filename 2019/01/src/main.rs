use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    println!("In file {}", filename);
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let line = line.unwrap();
        let puzzle_input = line.parse::<i32>().unwrap();
        let puzzle_output = solve(puzzle_input);
        println!("{}: {}", puzzle_input, puzzle_output);
    }
}

fn solve(input: i32) -> i32 {
    (input / 2) - 2
}