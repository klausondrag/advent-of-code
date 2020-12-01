use std::fs;

fn main() {
    let input_filename = "input.txt";
    let contents = fs::read_to_string(input_filename).unwrap();
    let final_register = solve_part1(contents);
    println!("Part1: Total fuel requirement: {}", final_register);
}

fn solve_part1(program: String) -> i32 {
    println!("{}", program);
    0
}

fn run_program(state: &mut Vec<usize>) -> &mut Vec<usize> {
    const OP_ADD: usize = 1;
    const OP_MULTIPLY: usize = 2;
    const OP_EXIT: usize = 99;
    let mut index = 0;
    let op_code = state[index];
    while op_code != OP_EXIT {
        if op_code == OP_ADD || op_code == OP_MULTIPLY {
            let a = state[index + 1];
            let b = state[index + 2];
            let target: usize = state[index + 3];
            let result;
            if op_code == OP_ADD {
                result = a + b;
            } else if op_code == OP_MULTIPLY {
                result = a * b;
            } else {
                panic!()
            }
            state[target] = result;
            index += 4;
        } else {
            panic!()
        }
    }

    state
}

#[cfg(test)]
mod tests {
    #[test]
    fn examples_part1() {
        use super::*;

        let cases: [(Vec<usize>, Vec<usize>); 4] = [
            (vec![1, 0, 0, 0, 99], vec![2, 0, 0, 0, 99]),
            (vec![2, 3, 0, 3, 99], vec![2, 3, 0, 6, 99]),
            (vec![2, 4, 4, 5, 99, 0], vec![2, 4, 4, 5, 99, 9801]),
            (
                vec![1, 1, 1, 4, 99, 5, 6, 0, 99],
                vec![30, 1, 1, 4, 2, 5, 6, 0, 99],
            ),
        ];

        for (mut input, mut target) in cases.iter() {
            println!("{:?} {:?}", input, target);
            let output = run_program(&mut input.to_owned());
            assert_eq!(*output, target);
        }
    }
}
