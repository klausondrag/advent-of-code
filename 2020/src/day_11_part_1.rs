use itertools::izip;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Copy, Clone, PartialEq)]
enum Space {
    EmptySeat,
    Floor,
    None,
    OccupiedSeat,
}

pub(crate) fn process(input_filename: &str) {
    let file = File::open(input_filename).unwrap();
    let reader = BufReader::new(file);

    let mut input = Vec::new();
    for line in reader.lines() {
        let line = line.unwrap();
        input.push(parse_line(line));
    }
    // -2 because None have already been inserted
    let row_length = input.get(0).unwrap().len() - 2;
    input.insert(0, vec![Space::None; row_length]);
    input.push(vec![Space::None; row_length]);

    let n_occupied_seats = solve(input, row_length);
    println!("Counted N occupied seats: {}", n_occupied_seats);
}

fn parse_line(line: String) -> Vec<Space> {
    let mut row: Vec<Space> = line
        .chars()
        .map(|c| match c {
            'L' => Space::EmptySeat,
            '.' => Space::Floor,
            '#' => Space::OccupiedSeat,
            _ => panic!(91110),
        })
        .collect();
    row.insert(0, Space::None);
    row.push(Space::None);
    row
}

fn solve(input: Vec<Vec<Space>>, row_length: usize) -> i32 {
    let n_rows = input.len();
    let mut old = input;
    let mut new = apply(old.clone(), n_rows, row_length);
    while has_changed(old, new.clone()) {
        old = new.clone();
        new = apply(old.clone(), n_rows, row_length);
    }

    count(new)
}

fn apply(old: Vec<Vec<Space>>, n_rows: usize, row_length: usize) -> Vec<Vec<Space>> {
    let mut new = Vec::new();
    for (old_row_index, old_row) in old.iter().enumerate().skip(1) {
        if old_row_index > n_rows - 2 {
            continue;
        }

        let mut new_row = vec![Space::None];
        for (old_column_index, old_seat) in old_row.iter().enumerate().skip(1) {
            if old_column_index > row_length {
                continue;
            }

            let new_seat: Space = match old_seat {
                Space::EmptySeat => {
                    if count_occupied_neighbours(old.clone(), old_row_index, old_column_index) == 0
                    {
                        Space::OccupiedSeat
                    } else {
                        Space::EmptySeat
                    }
                }
                Space::Floor => Space::Floor,
                Space::None => panic!(91111),
                Space::OccupiedSeat => {
                    if count_occupied_neighbours(old.clone(), old_row_index, old_column_index) >= 4
                    {
                        Space::EmptySeat
                    } else {
                        Space::OccupiedSeat
                    }
                }
            };
            new_row.push(new_seat);
        }

        new_row.push(Space::None);
        new.push(new_row);
    }
    new.insert(0, vec![Space::None; row_length]);
    new.push(vec![Space::None; row_length]);

    new
}

fn count_occupied_neighbours(input: Vec<Vec<Space>>, row_index: usize, column_index: usize) -> u8 {
    let mut n_occupied_seats: u8 = 0;

    for row in input.iter().skip(row_index - 1).take(3) {
        for seat in row.iter().skip(column_index - 1).take(3) {
            if *seat == Space::OccupiedSeat {
                n_occupied_seats += 1;
            }
        }
    }

    if *input.get(row_index).unwrap().get(column_index).unwrap() == Space::OccupiedSeat {
        n_occupied_seats -= 1;
    }

    n_occupied_seats
}

fn has_changed(a: Vec<Vec<Space>>, b: Vec<Vec<Space>>) -> bool {
    for (row_a, row_b) in izip!(a, b) {
        for (seat_a, seat_b) in izip!(row_a, row_b) {
            if seat_a != seat_b {
                return true;
            }
        }
    }

    false
}

fn count(input: Vec<Vec<Space>>) -> i32 {
    let mut n_occupied_seats = 0;
    for row in input {
        for seat in row {
            if seat == Space::OccupiedSeat {
                n_occupied_seats += 1;
            }
        }
    }

    n_occupied_seats
}

#[cfg(test)]
mod tests {
    #[test]
    fn example() {
        let texts = vec![
            String::from("L.LL.LL.LL"),
            String::from("LLLLLLL.LL"),
            String::from("L.L.L..L.."),
            String::from("LLLL.LL.LL"),
            String::from("L.LL.LL.LL"),
            String::from("L.LLLLL.LL"),
            String::from("..L.L....."),
            String::from("LLLLLLLLLL"),
            String::from("L.LLLLLL.L"),
            String::from("L.LLLLL.LL"),
        ];
        let row_length = texts.get(0).unwrap().len();
        let mut input: Vec<Vec<super::Space>> = texts
            .iter()
            .map(|t| super::parse_line(t.to_string()))
            .collect();
        input.insert(0, vec![super::Space::None; row_length]);
        input.push(vec![super::Space::None; row_length]);

        let solution = 37;
        let output = super::solve(input, row_length);
        assert_eq!(solution, output);
    }
}
