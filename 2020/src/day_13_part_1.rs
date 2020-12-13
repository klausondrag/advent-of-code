use std::fs::File;
use std::io::{BufRead, BufReader};

pub(crate) fn process(input_filename: &str) {
    let file = File::open(input_filename).unwrap();
    let reader = BufReader::new(file);

    let mut arrival_minute = None;
    let mut input_numbers = Vec::new();
    for line in reader.lines() {
        let line = line.unwrap();
        if arrival_minute.is_none() {
            arrival_minute = Some(line.parse::<i32>().unwrap());
        } else {
            let values = line.split(",");
            for v in values {
                match v.parse::<i32>() {
                    Ok(n) => input_numbers.push(n),
                    Err(_) => {}
                }
            }
        }
    }

    let solution = solve(input_numbers, arrival_minute.unwrap());

    match solution {
        Some((bus_id, departure_time)) => {
            println!(
                "Found Numbers.\n{} * ({} - {}) = {}",
                bus_id,
                departure_time,
                arrival_minute.unwrap(),
                bus_id * (departure_time - arrival_minute.unwrap())
            );
        }
        None => {
            panic!(91310);
        }
    }
}

fn solve(bus_ids: Vec<i32>, arrival_minute: i32) -> Option<(i32, i32)> {
    let float_arrival_minute = arrival_minute as f32;
    let mut min_departure: Option<(i32, i32)> = None;
    for bus_id in bus_ids {
        let float_bus_id = bus_id as f32;
        let next_available_departure =
            ((float_arrival_minute / float_bus_id).ceil() * float_bus_id) as i32;
        if min_departure.is_none() {
            min_departure = Some((bus_id, next_available_departure));
        } else {
            if next_available_departure < min_departure.unwrap().1 {
                min_departure = Some((bus_id, next_available_departure))
            }
        }
    }

    min_departure
}

#[cfg(test)]
mod tests {
    #[test]
    fn example() {
        const ARRIVAL_MINUTE: i32 = 939;
        let input_numbers = vec![7, 13, 59, 31, 19];
        let solution = 59 * (944 - 939); // = 295
        let output = super::solve(input_numbers, ARRIVAL_MINUTE);
        match output {
            Some((bus_id, departure_time)) => {
                assert_eq!(solution, bus_id * (departure_time - ARRIVAL_MINUTE))
            }
            None => assert!(false),
        }
    }
}
