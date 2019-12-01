#[cfg(test)] #[macro_use] extern crate cached;
#[cfg(test)] #[macro_use] extern crate timeit;

use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};


fn main() {
    let args: Vec<String> = env::args().collect();
    let input_filename = &args[1];

    let file = File::open(input_filename).unwrap();
    let reader = BufReader::new(file);

    let mut sum_part1 = 0;
    let mut sum_part2 = 0;
    for line in reader.lines() {
        let line = line.unwrap();
        let puzzle_input = line.parse::<i32>().unwrap();
        sum_part1 += solve_part1(puzzle_input);
        sum_part2 += solve_part2(puzzle_input);
    }

    println!("Part1: Total fuel requirement: {}", sum_part1);
    println!("Part2: Total fuel requirement: {}", sum_part2);
}

fn solve_part1(input: i32) -> i32 {
    (input / 3) - 2
}

fn solve_part2(input: i32) -> i32 {
    // for fibonacchi, caching requires 1/3 of the time
    // for this problem, caching requires 3x the time :(
    let required_fuel = (input / 3) - 2;
    if required_fuel <= 0 {
        0
    } else {
        let fuel_for_fuel = solve_part2(required_fuel);
        required_fuel + fuel_for_fuel
    }
}


#[cfg(test)]
mod tests {
    #[test]
    fn examples_part1() {
        use super::*;
        let inputs = vec![12, 14, 1969, 100756];
        let solutions = vec![2, 2, 654, 33583];
        let outputs: Vec<i32> = inputs.into_iter().map(solve_part1).collect();
        assert_eq!(solutions, outputs);
    }

    #[test]
    fn examples_part2() {
        use super::*;
        let inputs = vec![12, 14, 1969, 100756];
        let solutions = vec![2, 2, 966, 50346];
        let outputs: Vec<i32> = inputs.into_iter().map(solve_part2).collect();
        assert_eq!(solutions, outputs);
    }

    #[test]
    #[ignore]
    fn speed_naive() {
        fn solve_part2_naive(input: i32) -> i32 {
            let required_fuel = (input / 3) - 2;
            if required_fuel <= 0 {
                0
            } else {
                let fuel_for_fuel = solve_part2_naive(required_fuel);
                required_fuel + fuel_for_fuel
            }
        }
        let n_loops = 1_000_000;
        let naive_time = timeit_loops!(n_loops, {
            let inputs = vec![12, 14, 1969, 100756];
            let _outputs: Vec<i32> = inputs.into_iter().map(solve_part2_naive).collect();
        });
        println!("Time for {} loops naive : {}", n_loops, naive_time);
    }

    #[test]
    #[ignore]
    fn speed_cached() {
        cached!{
            PART2;
            fn solve_part2_cached(input: i32) -> i32 = {
                let required_fuel = (input / 3) - 2;
                if required_fuel <= 0 {
                    0
                } else {
                    let fuel_for_fuel = solve_part2_cached(required_fuel);
                    required_fuel + fuel_for_fuel
                }
            }
        }
        let n_loops = 1_000_000;
        let time = timeit_loops!(n_loops, {
            let inputs = vec![12, 14, 1969, 100756];
            let _outputs: Vec<i32> = inputs.into_iter().map(solve_part2_cached).collect();
        });
        println!("Time for {} loops cached: {}", n_loops, time);
    }
}