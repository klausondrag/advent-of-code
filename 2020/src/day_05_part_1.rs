use std::fs::File;
use std::io::{BufRead, BufReader};

pub(crate) fn process(input_filename: &str) {
    let file = File::open(input_filename).unwrap();
    let reader = BufReader::new(file);

    let mut highest_seat_id = -1;
    for line in reader.lines() {
        let line = line.unwrap();
        let found_seat_id = solve(line);
        if found_seat_id > highest_seat_id {
            highest_seat_id = found_seat_id;
        }
    }

    println!("Found highest seat id: {}", highest_seat_id);
}

fn solve(seat_instructions: String) -> i32 {
    assert_eq!(seat_instructions.len(), 10);
    const N_COLUMNS: i32 = 8;
    let mut lower_row_limit = 0;
    let mut upper_row_limit = 127;
    let mut lower_column_limit = 0;
    let mut upper_column_limit = N_COLUMNS - 1;
    for instruction in seat_instructions.chars() {
        let halfway_row = lower_row_limit + (upper_row_limit - lower_row_limit) / 2;
        let halfway_column = lower_column_limit + (upper_column_limit - lower_column_limit) / 2;
        match instruction {
            'F' => upper_row_limit = halfway_row,
            'B' => lower_row_limit = halfway_row + 1,
            'L' => upper_column_limit = halfway_column,
            'R' => lower_column_limit = halfway_column + 1,
            _ => panic!(90510),
        }
    }
    lower_row_limit * N_COLUMNS + lower_column_limit
}

#[cfg(test)]
mod tests {
    #[test]
    fn example() {
        let inputs = vec![
            String::from("BFFFBBFRRR"),
            String::from("FFFBBBFRRR"),
            String::from("BBFFBBFRLL"),
        ];
        let solutions = vec![567, 119, 820];
        let outputs: Vec<i32> = inputs.into_iter().map(super::solve).collect();
        assert_eq!(solutions, outputs);
    }
}
