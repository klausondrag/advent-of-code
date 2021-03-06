use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub(crate) fn process(input_filename: &str) {
    let file = File::open(input_filename).unwrap();
    let reader = BufReader::new(file);

    let mut n_answers = 0;
    let mut group_answers: Vec<String> = Vec::new();
    for line in reader.lines() {
        let line = line.unwrap();
        if line == "" {
            n_answers += solve(group_answers);
            group_answers = Vec::new();
        } else {
            group_answers.push(line);
        }
    }
    if group_answers.len() > 0 {
        n_answers += solve(group_answers);
    }

    println!("Sum of the counts: {}", n_answers);
}

fn solve(answers: Vec<String>) -> usize {
    let mut group_answers = HashSet::new();
    for (index, answer) in answers.iter().enumerate() {
        let mut person_answers: HashSet<char> = HashSet::new();
        for char in answer.chars() {
            person_answers.insert(char);
        }
        if index == 0 {
            group_answers = person_answers;
        } else {
            group_answers = group_answers
                .intersection(&person_answers)
                .copied()
                .collect();
        }
    }

    group_answers.len()
}

#[cfg(test)]
mod tests {
    #[test]
    fn example() {
        let inputs = vec![
            vec![String::from("abc")],
            vec![String::from("a"), String::from("b"), String::from("c")],
            vec![String::from("ab"), String::from("ac")],
            vec![
                String::from("a"),
                String::from("a"),
                String::from("a"),
                String::from("a"),
            ],
            vec![String::from("b")],
        ];
        let solutions = vec![3, 0, 1, 1, 1]; // = 6
        let outputs: Vec<usize> = inputs.into_iter().map(super::solve).collect();
        assert_eq!(solutions, outputs);
    }
}
