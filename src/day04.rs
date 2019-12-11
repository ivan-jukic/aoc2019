// Day 4 AoC 2019

use crate::utils::str_vec_to_i32_vec;


type Range = (i32, i32);

enum Part { First, Second }


#[aoc_generator(day4)]
fn get_range(input: &str) -> Range {

    // This deconstruct is pretty cool!
    match str_vec_to_i32_vec(input, "-").as_slice() {
        [hi, lo] => (hi.clone(), lo.clone()),        
        _        => panic!("Day 4 input is in unrecognisable format!") 
    }
}


#[aoc(day4, part1)]
fn part1((lo, hi): &Range) -> i32 {
    let mut count = 0;
    for pass in *lo .. (*hi + 1) {
        if is_valid_pass(pass, Part::First) {
            count += 1;
        }
    }

    count
}


// A bit of a repetition
#[aoc(day4, part2)]
fn part2((lo, hi): &Range) -> i32 {
    let mut count = 0;
    for pass in *lo .. (*hi + 1) {
        if is_valid_pass(pass, Part::Second) {
            count += 1;
        }
    }

    count
}


// Pass validation fn's


fn is_valid_pass(pass: i32, part: Part) -> bool {
    let is_six_digits: bool = pass > 99999 && pass < 1000000;
    let pass_as_chars: Vec<char> = pass.to_string().chars().collect();

    let has_two_adjacent_same_digits =
        match part {
            Part::First  => test_adjacent_digits(pass_as_chars.clone()),
            Part::Second => test_two_adjacent_digits(pass_as_chars.clone(), vec!())
        };

    let is_in_ascending_order =
        test_ascending_digits(pass_as_chars.clone());

    is_six_digits && has_two_adjacent_same_digits && is_in_ascending_order
}


fn test_adjacent_digits(mut digits: Vec<char>) -> bool {
    if digits.len() > 1 {
        let first: char = *(digits.get(0).unwrap());
        let second: char = *(digits.get(1).unwrap());

        if first == second {
            true
        } else {
            digits.remove(0);
            test_adjacent_digits(digits)
        }
    } else {
        false
    }
}


fn test_ascending_digits(mut digits: Vec<char>) -> bool {
    if digits.len() > 1 {
        let first: char = *(digits.get(0).unwrap());
        let second: char = *(digits.get(1).unwrap());

        if first > second {
            false
        } else {
            digits.remove(0);
            test_ascending_digits(digits)
        }
    } else {
        true
    }
}


fn test_two_adjacent_digits(mut digits: Vec<char>, mut group: Vec<char>) -> bool {
    if digits.len() > 0 {
        let num = digits.remove(0);
        let num_in_group = group.contains(&num);
        
        if group.len() == 0 || num_in_group {
            group.push(num);
        }

        // If a group of two adjacent digits was found!
        if group.len() == 2 && (digits.len() == 0 || !num_in_group) {
            true

        } else {

            // Need to check again here...
            if !group.contains(&num) {
                group.clear();
                group.push(num);
            };

            test_two_adjacent_digits(digits, group)
        }
    } else {
        false
    }
}


#[cfg(test)]
mod tests {
    use super::{Part, get_range, is_valid_pass, test_adjacent_digits, test_ascending_digits, test_two_adjacent_digits};

    #[test]
    fn day4_input_test1() {
        assert_eq!(get_range("10-20"), (10, 20));
        assert_eq!(get_range("123123-4564564"), (123123, 4564564));
        assert_ne!(get_range("25-1023"), (10, 20));
    }

    #[test]
    fn day4_pass_digits_adjacent_test() {
        assert_eq!(test_adjacent_digits(vec!('2', '2')), true);
        assert_eq!(test_adjacent_digits(vec!('1', '2', '3', '3', '4', '5')), true);
        assert_eq!(test_adjacent_digits(vec!('1', '2', '3')), false);
        assert_eq!(test_adjacent_digits(vec!('7', '8', '9')), false);
    }

    #[test]
    fn day4_pass_digits_ascending_test() {
        assert_eq!(test_ascending_digits(vec!('1', '2', '3', '4', '5', '5')), true);
        assert_eq!(test_ascending_digits(vec!('7', '8', '9', '1', '2', '3')), false);
    }

    #[test]
    fn day4_pass2_two_adjacent_test() {
        assert_eq!(test_two_adjacent_digits(vec!('1', '1', '2', '2', '3', '3'), vec!()), true);
        assert_eq!(test_two_adjacent_digits(vec!('1', '2', '3', '4', '4', '4'), vec!()), false);
        assert_eq!(test_two_adjacent_digits(vec!('1', '1', '1', '1', '2', '2'), vec!()), true);
    }

    #[test]
    fn day4_pass_valid_test() {
        assert_eq!(is_valid_pass(111111, Part::First), true);
        assert_eq!(is_valid_pass(123389, Part::First), true);
        assert_eq!(is_valid_pass(223450, Part::First), false);
        assert_eq!(is_valid_pass(123789, Part::First), false);
        assert_eq!(is_valid_pass(1233489, Part::First), false);

        assert_eq!(is_valid_pass(112233, Part::Second), true);
        assert_eq!(is_valid_pass(123444, Part::Second), false);
        assert_eq!(is_valid_pass(111122, Part::Second), true);
    }
}
