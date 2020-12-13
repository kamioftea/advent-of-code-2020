//! This is my solution for [Advent of Code - Day 2](https://adventofcode.com/2020/day/2) -
//! _Password Philosophy_
//!
//! Today's challenges were themed around validating data. The puzzle input was a 1000 lines in a
//! specific format. The first part of each line could be interpreted as a policy that the rest of
//! the line is expected to match. The lines could be parsed with a regular expression, and the
//! policies could be implemented as function `(policy: &Policy, password: &str) -> bool`. The
//! built-in rust iterator functions are then suitable for reducing the input data to a count of the
//! valid lines.

use regex::Regex;
use std::fs;

/// The entry point for running the solutions with the 'real' puzzle input.
///
/// - The puzzle input is expected to be at `<project_root>/res/day-2-input`
/// - It is expected this will be called by [`super::main()`] when the user elects to run day 2.
pub fn run() {
    let contents = fs::read_to_string("res/day-2-input").expect("Failed to read file");
    let lines_sr = contents.lines();
    let count_sr = lines_sr.flat_map(|line| parse_line(line))
        .filter(|(policy, password)| is_valid_for_part_1(policy, password))
        .count();
    println!("There were {} valid sled rental lines", count_sr);

    let lines_ot = contents.lines();
    let count_ot = lines_ot.flat_map(|line| parse_line(line))
        .filter(|(policy, password)| is_valid_for_part_2(policy, password))
        .count();
    println!("There were {} valid Official Toboggan lines", count_ot);
}

/// Holds the policy variables from an input line
#[derive(Debug, Eq, PartialEq)]
struct Policy {
    min: usize,
    max: usize,
    letter: char,
}

/// Parses an input line into the policy data, and the string to validate
///
/// Each line has a standard format:
/// ```
/// 1-3 a: abcde
/// 1-3 b: cdefg
/// 12-19 c: ccccccccc
/// ```
/// This uses a regular expression to extract both numbers and the letter and map these to a
/// [`Policy`], and the string data that should match the policy.
///
/// # Examples from text
/// ```
/// assert_eq!(parse_line("1-3 a: abcde"), Some((Policy { min: 1, max: 3, letter: 'a' }, "abcde")));
/// assert_eq!(parse_line("1-3 b: cdefg"), Some((Policy { min: 1, max: 3, letter: 'b' }, "cdefg")));
/// assert_eq!(parse_line("2-9 c: ccccccccc"), Some((Policy { min: 2, max: 9, letter: 'c' }, "ccccccccc")));
/// assert_eq!(parse_line("29 c: ccccccccc"), None);
/// ```
fn parse_line(line: &str) -> Option<(Policy, &str)> {
    let re = Regex::new(r"^(\d+)-(\d+) ([a-z]): ([a-z]+)$").unwrap();
    match re.captures(line) {
        Some(m) => Some((
            Policy {
                min: m.get(1).unwrap().as_str().parse::<usize>().unwrap(),
                max: m.get(2).unwrap().as_str().parse::<usize>().unwrap(),
                letter: m.get(3).unwrap().as_str().parse::<char>().unwrap(),
            },
            m.get(4).unwrap().as_str()
        )),
        _ => None
    }
}

/// The solution to part 1
///
/// > Each line gives the password policy and then the password. The password policy indicates the
/// > lowest and highest number of times a given letter must appear for the password to be valid.
/// > For example, 1-3 a means that the password must contain a at least 1 time and at most 3 times.
///
/// # Examples from text
/// ```
/// assert_eq!(parse_line("1-3 a: abcde"), Some((Policy { min: 1, max: 3, letter: 'a' }, "abcde")));
/// assert_eq!(parse_line("1-3 b: cdefg"), Some((Policy { min: 1, max: 3, letter: 'b' }, "cdefg")));
/// assert_eq!(parse_line("2-9 c: ccccccccc"), Some((Policy { min: 2, max: 9, letter: 'c' }, "ccccccccc")));
/// assert_eq!(parse_line("29 c: ccccccccc"), None);
/// ```
fn is_valid_for_part_1(policy: &Policy, password: &str) -> bool {
    let count = password.chars().filter(|&c| c == policy.letter).count();

    return count >= policy.min && count <= policy.max;
}

/// The solution to part 2
///
/// > Each policy actually describes two positions in the password, where 1 means the first
/// > character, 2 means the second character, and so on. (Be careful; Toboggan Corporate Policies
/// > have no concept of "index zero"!) Exactly one of these positions must contain the given
/// > letter. Other occurrences of the letter are irrelevant for the purposes of policy enforcement.
///
/// # Examples from text
/// ```
/// assert_eq!(is_valid_ot(&Policy { min: 1, max: 3, letter: 'a' }, "abcde"), true);
/// assert_eq!(is_valid_ot(&Policy { min: 1, max: 3, letter: 'b' }, "cdefg"), false);
/// assert_eq!(is_valid_ot(&Policy { min: 2, max: 9, letter: 'c' }, "ccccccccc"), false);
/// ```
fn is_valid_for_part_2(policy: &Policy, password: &str) -> bool {
    if password.len() < policy.max as usize {
        return false
    }

    let mut chars = password.chars();

    let a = chars.nth(policy.min - 1).unwrap();
    let b = chars.nth(policy.max - policy.min - 1).unwrap();

    return a != b && (a == policy.letter || b == policy.letter)
}

#[cfg(test)]
mod tests {
    use day_2::{parse_line, Policy, is_valid_for_part_1, is_valid_for_part_2};

    //noinspection SpellCheckingInspection
    #[test]
    fn can_parse_line() {
        assert_eq!(parse_line("1-3 a: abcde"), Some((Policy { min: 1, max: 3, letter: 'a' }, "abcde")));
        assert_eq!(parse_line("1-3 b: cdefg"), Some((Policy { min: 1, max: 3, letter: 'b' }, "cdefg")));
        assert_eq!(parse_line("2-9 c: ccccccccc"), Some((Policy { min: 2, max: 9, letter: 'c' }, "ccccccccc")));
        assert_eq!(parse_line("29 c: ccccccccc"), None);
    }

    //noinspection SpellCheckingInspection
    #[test]
    fn can_validate_sled_rental() {
        assert_eq!(is_valid_for_part_1(&Policy { min: 1, max: 3, letter: 'a' }, "abcde"), true);
        assert_eq!(is_valid_for_part_1(&Policy { min: 1, max: 3, letter: 'b' }, "cdefg"), false);
        assert_eq!(is_valid_for_part_1(&Policy { min: 2, max: 9, letter: 'c' }, "ccccccccc"), true);
    }

    //noinspection SpellCheckingInspection
    #[test]
    fn can_validate_official_toboggan() {
        assert_eq!(is_valid_for_part_2(&Policy { min: 1, max: 3, letter: 'a' }, "abcde"), true);
        assert_eq!(is_valid_for_part_2(&Policy { min: 1, max: 3, letter: 'b' }, "cdefg"), false);
        assert_eq!(is_valid_for_part_2(&Policy { min: 2, max: 9, letter: 'c' }, "ccccccccc"), false);
    }
}
