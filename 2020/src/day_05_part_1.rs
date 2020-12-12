use std::fs::File;
use std::io::{BufRead, BufReader};

pub(crate) fn process(input_filename: &str) {
    let file = File::open(input_filename).unwrap();
    let reader = BufReader::new(file);

    let mut highest_seat_id = -1;
    for line in reader.lines() {
        let line = line.unwrap();
        let found_seat_id = solve1(line);
        if found_seat_id > highest_seat_id {
            highest_seat_id = found_seat_id;
        }
    }

    println!("Found highest seat id: {}", highest_seat_id);
}

fn solve1(seat_instructions: String) -> i32 {
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

fn solve2(seat_instructions: String) -> i32 {
    assert_eq!(seat_instructions.len(), 10);
    let mut row = 0;
    let mut column = 0;
    for instruction in seat_instructions.chars() {
        match instruction {
            'F' => row <<= 1,
            'B' => row = (row << 1) | 1,
            'L' => column <<= 1,
            'R' => column = (column << 1) | 1,
            _ => panic!(90511),
        }
    }
    return row * 8 + column
}

fn solve3(seat_instructions: String) -> i32 {
    assert_eq!(seat_instructions.len(), 10);
    let mut seat: u16 = 0;
    for (index, instruction) in seat_instructions.chars().enumerate() {
        match instruction {
            'B' | 'R' => {
                seat |= 1 << (9 - index);
            },
            'F' | 'L' => {},
            _ => panic!(90512),

        }
    }
    seat as i32
}

#[cfg(test)]
mod tests {
    #[test]
    fn examples() {
        let inputs = vec![
            String::from("BFFFBBFRRR"),
            String::from("FFFBBBFRRR"),
            String::from("BBFFBBFRLL"),
        ];
        let solutions = vec![567, 119, 820];
        let outputs1: Vec<i32> = inputs.clone().into_iter().map(super::solve1).collect();
        let outputs2: Vec<i32> = inputs.clone().into_iter().map(super::solve2).collect();
        let outputs3: Vec<i32> = inputs.clone().into_iter().map(super::solve3).collect();
        assert_eq!(solutions, outputs1);
        assert_eq!(solutions, outputs2);
        assert_eq!(solutions, outputs3);
    }
}
