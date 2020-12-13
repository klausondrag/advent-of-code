use itertools::izip;
use std::fs::File;
use std::io::{BufRead, BufReader};

struct Bus {
    id: i64,
    offset: i64,
}

pub(crate) fn process(input_filename: &str) {
    let file = File::open(input_filename).unwrap();
    let reader = BufReader::new(file);

    let mut arrival_minute = None;
    let mut input = Vec::new();
    for line in reader.lines() {
        let line = line.unwrap();
        if arrival_minute.is_none() {
            // The first line in your input is no longer relevant.
            arrival_minute = Some(line);
        } else {
            let values = line.split(",");
            for (index, v) in values.enumerate() {
                match v.parse::<i64>() {
                    Ok(n) => input.push(Bus {
                        id: n,
                        offset: index as i64,
                    }),
                    Err(_) => {}
                }
            }
        }
    }

    let solution = solve(input);
    println!("Found departure time: {}", solution);
}

fn solve(input: Vec<Bus>) -> i64 {
    /* Find t1
    t_1 = 7 * x_1 - 0
    t_1 = 13 * x_2 - 1

    t_1 = id1 * x_1 - o_1
    t_1 = id2 * x_2 - o_2

    (t_1 + o_1) % id_1 ≡ 0
    (t_1 + o_2) % id_2 ≡ 0

    ids are prime so we can use chinese remainder theorem
    https://brilliant.org/wiki/chinese-remainder-theorem/
    which states
    let N = id_1 * id_2 * ...
    let y_i = N / id_i
    let z_i ≡ y_i^-1 % id_i
    let x = sum a_i * y_i * z_i
    let solution = x % N

    z_i, x, and solution are implemented slightly different
    based on https://gist.github.com/miseran/abf1629c6498538a0175ff7548635317
     */

    let n: i64 = input.iter().map(|b| b.id).product();
    let ys: Vec<i64> = input.iter().map(|b| n / b.id).collect();
    let zs: Vec<i64> = input
        .iter()
        .zip(ys.iter())
        .map(|(b, y)| (inverse_modulo(*y, b.id)) as i64)
        .collect();
    let x: i64 = izip!(input.iter(), ys.iter(), zs.iter())
        .map(|(b, y, z)| -b.offset * y * z)
        .sum();
    let solution: i64 = x.rem_euclid(n);
    solution
}

fn inverse_modulo(x: i64, p: i64) -> i64 {
    // p must be prime
    (0..p - 2).fold(1, |o, _| (o * x) % p)
}

#[cfg(test)]
mod tests {
    #[test]
    fn example() {
        let inputs = vec![
            vec![
                super::Bus { id: 7, offset: 0 },
                super::Bus { id: 13, offset: 1 },
                super::Bus { id: 59, offset: 4 },
                super::Bus { id: 31, offset: 6 },
                super::Bus { id: 19, offset: 7 },
            ],
            vec![
                super::Bus { id: 17, offset: 0 },
                super::Bus { id: 13, offset: 2 },
                super::Bus { id: 19, offset: 3 },
            ],
            vec![
                super::Bus { id: 67, offset: 0 },
                super::Bus { id: 7, offset: 1 },
                super::Bus { id: 59, offset: 2 },
                super::Bus { id: 61, offset: 3 },
            ],
            vec![
                super::Bus { id: 67, offset: 0 },
                super::Bus { id: 7, offset: 2 },
                super::Bus { id: 59, offset: 3 },
                super::Bus { id: 61, offset: 4 },
            ],
            vec![
                super::Bus { id: 67, offset: 0 },
                super::Bus { id: 7, offset: 1 },
                super::Bus { id: 59, offset: 3 },
                super::Bus { id: 61, offset: 4 },
            ],
            vec![
                super::Bus {
                    id: 1789,
                    offset: 0,
                },
                super::Bus { id: 37, offset: 1 },
                super::Bus { id: 47, offset: 2 },
                super::Bus {
                    id: 1889,
                    offset: 3,
                },
            ],
        ];
        let solutions = vec![1068781, 3417, 754018, 779210, 1261476, 1202161486];
        let outputs: Vec<i64> = inputs.into_iter().map(super::solve).collect();
        assert_eq!(solutions, outputs)
    }
}
