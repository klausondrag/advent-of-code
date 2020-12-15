use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub(crate) fn process(input_filename: &str) {
    let file = File::open(input_filename).unwrap();
    let reader = BufReader::new(file);

    let mut input_numbers = Vec::new();
    for line in reader.lines() {
        let line = line.unwrap();
        let input_number = line.parse::<usize>().unwrap();
        input_numbers.push(input_number);
    }

    const NTH_SPOKEN_NUMBER: usize = 2020;
    let solution = solve(input_numbers, NTH_SPOKEN_NUMBER);
    println!("The {}th spoken number is: {}", NTH_SPOKEN_NUMBER, solution);
}

fn solve(input_numbers: Vec<usize>, nth_spoken_number: usize) -> usize {
    let mut number_to_turn: HashMap<usize, usize> = HashMap::new();

    for (index, number) in input_numbers
        .iter()
        .enumerate()
        .take(input_numbers.len() - 1)
    {
        // turns start with 1
        number_to_turn.insert(*number, index + 1);
    }

    let mut previous_turn = input_numbers.len();
    let mut spoken_number_previous_turn = *input_numbers.get(input_numbers.len() - 1).unwrap();

    while previous_turn < nth_spoken_number {
        let spoken_number_this_turn = match number_to_turn.get(&spoken_number_previous_turn) {
            None => 0,
            Some(last_turn_when_spoken) => previous_turn - *last_turn_when_spoken,
        };

        number_to_turn.insert(spoken_number_previous_turn, previous_turn);
        spoken_number_previous_turn = spoken_number_this_turn;
        previous_turn += 1;
    }

    spoken_number_previous_turn
}

#[cfg(test)]
mod tests {
    #[test]
    fn example() {
        const NTH_SPOKEN_NUMBER: usize = 2020;
        let inputs: Vec<Vec<usize>> = vec![
            vec![0, 3, 6],
            vec![1, 3, 2],
            vec![2, 1, 3],
            vec![1, 2, 3],
            vec![2, 3, 1],
            vec![3, 2, 1],
            vec![3, 1, 2],
        ];
        let solutions: Vec<usize> = vec![436, 1, 10, 27, 78, 438, 1836];
        let outputs: Vec<usize> = inputs
            .iter()
            .map(|input_numbers| super::solve(input_numbers.to_vec(), NTH_SPOKEN_NUMBER))
            .collect();
        assert_eq!(solutions, outputs)
    }
}
