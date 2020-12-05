use itertools::multizip;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::process::exit;

pub(crate) fn process(input_filename: &str) {
    let file = File::open(input_filename).unwrap();
    let reader = BufReader::new(file);

    let mut seat_instructions = Vec::new();
    for line in reader.lines() {
        let line = line.unwrap();
        seat_instructions.push(line);
    }

    let seat_ids: Vec<usize> = seat_instructions.into_iter().map(convert).collect();
    let my_seat_id = solve(seat_ids);

    match my_seat_id {
        Some(my_seat_id) => println!("My seat id: {}", my_seat_id),
        None => {
            println!("Error: No solution found");
            // error specification: 9XXYZ for the Z-th error of day XX part Y
            exit(90520);
        }
    }
}

fn convert(seat_instructions: String) -> usize {
    assert_eq!(seat_instructions.len(), 10);
    const N_COLUMNS: usize = 8;
    let mut lower_row_limit: usize = 0;
    let mut upper_row_limit: usize = 127;
    let mut lower_column_limit: usize = 0;
    let mut upper_column_limit: usize = N_COLUMNS - 1;
    for instruction in seat_instructions.chars() {
        let halfway_row = lower_row_limit + (upper_row_limit - lower_row_limit) / 2;
        let halfway_column = lower_column_limit + (upper_column_limit - lower_column_limit) / 2;
        match instruction {
            'F' => upper_row_limit = halfway_row,
            'B' => lower_row_limit = halfway_row + 1,
            'L' => upper_column_limit = halfway_column,
            'R' => lower_column_limit = halfway_column + 1,
            _ => panic!(90520),
        }
    }
    lower_row_limit * N_COLUMNS + lower_column_limit
}

fn solve(seat_ids: Vec<usize>) -> Option<usize> {
    let max_value = *seat_ids.iter().max().unwrap();
    let mut occupancy_vector = vec![true; max_value + 1];
    for seat in seat_ids {
        occupancy_vector[seat] = false;
    }

    for (
        left_neighbour_seat_index,
        (left_neighbour_availability, my_seat_availability, right_neighbour_availability),
    ) in multizip((
        occupancy_vector.iter(),
        occupancy_vector.iter().skip(1),
        occupancy_vector.iter().skip(2),
    ))
    .enumerate()
    {
        if !*left_neighbour_availability && *my_seat_availability && !*right_neighbour_availability
        {
            return Some(left_neighbour_seat_index + 1);
        }
    }

    None
}

#[cfg(test)]
mod tests {
    #[test]
    fn example() {
        let inputs = vec![0, 1, 5, 41, 43, 1016, 1017, 1021];
        let solution = 42;
        let output = super::solve(inputs);
        match output {
            Some(output) => assert_eq!(solution, output),
            None => assert!(false),
        }
    }
}
