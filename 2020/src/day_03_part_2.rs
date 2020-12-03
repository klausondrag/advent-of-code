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

    let down_movements: Vec<usize> = vec![1, 1, 1, 1, 2];
    let right_movements: Vec<usize> = vec![1, 3, 5, 7, 1];

    let mut result = 1;
    for (down, right) in down_movements.iter().zip(right_movements.iter()) {
        let n_trees = solve(&input, *down, *right);
        println!(
            "Encountered n trees for slope (down={}, right={}): {}",
            *down, *right, n_trees,
        );
        result *= n_trees;
    }

    println!("Result: {}", result);
}

fn solve(input: &Vec<String>, down_movement: usize, right_movement: usize) -> i32 {
    const TREE_CHARACTER: char = '#';

    let mut n_trees = 0;
    for (index, row) in input.iter().step_by(down_movement).enumerate() {
        let x_position = (index * right_movement) % row.chars().count();
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
        let down_movements = vec![1, 1, 1, 1, 2];
        let right_movements = vec![1, 3, 5, 7, 1];
        let solutions = vec![2, 7, 3, 4, 2]; // = 336
        let outputs: Vec<i32> = down_movements
            .iter()
            .zip(right_movements.iter())
            .map(|(down, right)| super::solve(&input, *down, *right))
            .collect();
        assert_eq!(solutions, outputs);
    }
}
