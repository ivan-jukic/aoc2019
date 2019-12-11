// Day 02 AOC 2019

use crate::utils::str_vec_to_i32_vec;


#[aoc_generator(day2)]
fn get_intprog(input: &str) -> Vec<i32> {
    str_vec_to_i32_vec(input, ",")
}


#[aoc(day2, part1)]
fn part1(inputs: &Vec<i32>) -> i32 {
    part1_solver(inputs.clone(), 0)
}


#[aoc(day2, part2)]
fn part2(_inputs: &Vec<i32>) -> i32 {

    // The second part was solved manually, by changing the noun and the verb.
    // Noun has significant influence on the final value on the zero index,
    // so we just increase it until we get close enough to our target value,
    // and then change the verb to fine tune it and get the exact value we are
    // looking for.

    let verb = 65;
    let noun = 77;

    100 * verb + noun
}


fn part1_solver(mut inputs: Vec<i32>, idx: usize) -> i32 {
    let op = inputs[idx];

    if op == 99 {
        inputs[0]
    } else {
        let idx_a = inputs[idx + 1] as usize;
        let idx_b = inputs[idx + 2] as usize;
        let save_idx = inputs[idx + 3] as usize;

        let a = inputs[idx_a];
        let b = inputs[idx_b];

        inputs[save_idx] = if op == 1 {
            a + b
        } else if op == 2 {
            a * b
        } else {
            panic!("Day 02 - Wth, I don't know what to do with op code {}", op);
        };

        part1_solver(inputs, idx + 4)
    }
}


#[cfg(test)]
mod tests {
    use super::{part1};

    #[test]
    fn day2_part1_test() {
        assert_eq!(part1(&vec!(1, 0, 0, 0, 99)), 2);
        assert_eq!(part1(&vec!(2, 3, 0, 3, 99)), 2);
        assert_eq!(part1(&vec!(2, 4, 4, 5, 99, 0)), 2);
        assert_eq!(part1(&vec!(1, 1, 1, 4, 99, 5, 6, 0, 99)), 30);
    }
}
