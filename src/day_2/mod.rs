use regex::Regex;
use std::fs;

pub fn run() {
    let contents = fs::read_to_string("res/day-2-input").expect("Failed to read file");
    let lines_sr = contents.lines();
    let count_sr = lines_sr.flat_map(|line| parse_line(line))
        .filter(|(policy, password)| is_valid_sr(policy, password))
        .count();
    println!("There were {} valid sled rental lines", count_sr);

    let lines_ot = contents.lines();
    let count_ot = lines_ot.flat_map(|line| parse_line(line))
        .filter(|(policy, password)| is_valid_ot(policy, password))
        .count();
    println!("There were {} valid Official Toboggan lines", count_ot);
}

#[derive(Debug, Eq, PartialEq)]
struct Policy {
    min: usize,
    max: usize,
    letter: char,
}

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

fn is_valid_sr(policy: &Policy, password: &str) -> bool {
    let count = password.chars().filter(|&c| c == policy.letter).count();

    return count >= policy.min && count <= policy.max;
}

fn is_valid_ot(policy: &Policy, password: &str) -> bool {
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
    use day_2::{parse_line, Policy, is_valid_sr, is_valid_ot};

    #[test]
    fn can_parse_line() {
        assert_eq!(parse_line("1-3 a: abcde"), Some((Policy { min: 1, max: 3, letter: 'a' }, "abcde")));
        assert_eq!(parse_line("1-3 b: cdefg"), Some((Policy { min: 1, max: 3, letter: 'b' }, "cdefg")));
        assert_eq!(parse_line("2-9 c: ccccccccc"), Some((Policy { min: 2, max: 9, letter: 'c' }, "ccccccccc")));
        assert_eq!(parse_line("29 c: ccccccccc"), None);
    }

    #[test]
    fn can_validate_sled_rental() {
        assert_eq!(is_valid_sr(&Policy { min: 1, max: 3, letter: 'a' }, "abcde"), true);
        assert_eq!(is_valid_sr(&Policy { min: 1, max: 3, letter: 'b' }, "cdefg"), false);
        assert_eq!(is_valid_sr(&Policy { min: 2, max: 9, letter: 'c' }, "ccccccccc"), true);
    }

    #[test]
    fn can_validate_official_toboggan() {
        assert_eq!(is_valid_ot(&Policy { min: 1, max: 3, letter: 'a' }, "abcde"), true);
        assert_eq!(is_valid_ot(&Policy { min: 1, max: 3, letter: 'b' }, "cdefg"), false);
        assert_eq!(is_valid_ot(&Policy { min: 2, max: 9, letter: 'c' }, "ccccccccc"), false);
    }
}
