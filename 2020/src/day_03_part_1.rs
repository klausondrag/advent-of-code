use std::fs::File;
use std::io::{BufRead, BufReader};

pub(crate) fn process(input_filename: &str) {
    let file = File::open(input_filename).unwrap();
    let reader = BufReader::new(file);

    let mut input = Vec::new();
    for line in reader.lines() {
        let line = line.unwrap();
        input.push(line);
    }

    let n_trees = solve(input);

    println!("Encountered n trees: {}", n_trees);
}

fn solve(input: Vec<String>) -> i32 {
    const RIGHT_MOVEMENT: usize = 3;
    const TREE_CHARACTER: char = '#';

    let mut n_trees = 0;
    for (index, row) in input.iter().enumerate() {
        let x_position = (index * RIGHT_MOVEMENT) % row.chars().count();
        let character = row.chars().nth(x_position).unwrap();
        if character == TREE_CHARACTER {
            n_trees += 1;
        }
    }

    n_trees
}

#[cfg(test)]
mod tests {
    #[test]
    fn example() {
        let input = vec![
            String::from("..##......."),
            String::from("#...#...#.."),
            String::from(".#....#..#."),
            String::from("..#.#...#.#"),
            String::from(".#...##..#."),
            String::from("..#.##....."),
            String::from(".#.#.#....#"),
            String::from(".#........#"),
            String::from("#.##...#..."),
            String::from("#...##....#"),
            String::from(".#..#...#.#"),
        ];
        let solution = 7;
        let output = super::solve(input);
        assert_eq!(solution, output);
    }
}
