// Day 01 AOC 2019

#[aoc_generator(day1)]
fn get_masses(input: &str) -> Vec<i32> {
    input
        .lines()
        .map(|m| m.trim())
        .map(|m| m.parse::<i32>().unwrap())
        .collect()
}


#[aoc(day1, part1, Vec<i32>)]
fn part1(input: &Vec<i32>) -> i32 {
    input
        .iter()
        .map(|m| fuel_calc(*m))
        .sum()
}


#[aoc(day1, part2, Vec<i32>)]
fn part2(input: &Vec<i32>) -> i32 {
    input
        .iter()
        .map(|m| calc_required_fuel(*m, 0))
        .sum()
}


fn calc_required_fuel(m: i32, tot: i32) -> i32 {
    let m2 = fuel_calc(m);

    if m2 > 0 {
        calc_required_fuel(m2, tot + m2)
    } else {
        tot
    }
}


fn fuel_calc(m: i32) -> i32 {
    m / 3 - 2
}


#[cfg(test)]
mod tests {
    use super::{part1};

    #[test]
    fn day1_part1_test1() {
        assert_eq!(part1(&vec!(12, 14)), 4);
    }

    #[test]
    fn day1_part1_test2() {
        assert_eq!(part1(&vec!(12, 14, 1969)), 658);
    }

    #[test]
    fn day1_part1_test3() {
        assert_eq!(part1(&vec!(12, 14, 1969, 100756)), 34241);
    }
}
