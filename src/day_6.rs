use std::fs;
use std::collections::HashSet;
use std::hash::Hash;

pub fn run() {
    let contents = fs::read_to_string("res/day-6-input").expect("Failed to read file");

    let union_groups = sum_counts(&parse_union_groups(contents.as_str()));
    println!("Sum of union group counts: {}", union_groups);

    let intersect_groups = sum_counts(&parse_intersect_groups(contents.as_str()));
    println!("Sum of intersect group counts: {}", intersect_groups);
}

fn parse_union_groups(input: &str) -> Vec<HashSet<char>> {
    input.split("\n\n").into_iter().map(|str| union_group_from_lines(str)).collect()
}

fn union_group_from_lines(lines: &str) -> HashSet<char> {
    let mut group = HashSet::new();

    lines.chars()
        .filter(|chr| ('a'..='z').contains(chr))
        .for_each(|chr| { group.insert(chr); });

    group
}

fn parse_intersect_groups(input: &str) -> Vec<HashSet<char>> {
    input.split("\n\n").into_iter().map(|str| intersect_group_from_lines(str)).collect()
}

fn intersect<T: Hash + Eq + Copy>(a: HashSet<T>, b: HashSet<T>) -> HashSet<T> {
    let mut out = HashSet::new();

    a.iter().filter(|&t| b.contains(t) ).for_each(|&t| {out.insert(t);});

    out
}

fn intersect_group_from_lines(lines: &str) -> HashSet<char> {
    let mut iter = lines.lines().map(|line| union_group_from_lines(line)).into_iter();
    let mut group = iter.next().unwrap();
    for i in iter {
        group = intersect(group, i)
    }

    group
}

fn sum_counts(groups: &Vec<HashSet<char>>) -> usize {
    groups.iter().map(|group| group.len()).sum()
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;
    use day_6::{union_group_from_lines, parse_union_groups, sum_counts, intersect_group_from_lines, parse_intersect_groups};

    //noinspection SpellCheckingInspection
    #[test]
    fn can_parse_union_group() {
        let input = "abcx
abcy
abcz";
        let expected_set: HashSet<char> = vec!('a', 'b', 'c', 'x', 'y', 'z').into_iter().collect();

        assert_eq!(
            expected_set,
            union_group_from_lines(input)
        );
    }

    #[test]
    fn can_parse_and_count_union_groups() {
        let input = "abc

a
b
c

ab
ac

a
a
a
a

b";
        let expected_groups: Vec<HashSet<char>> = vec!(
            vec!('a', 'b', 'c').into_iter().collect(),
            vec!('a', 'b', 'c').into_iter().collect(),
            vec!('a', 'b', 'c').into_iter().collect(),
            vec!('a').into_iter().collect(),
            vec!('b').into_iter().collect(),
        );

        let actual_groups = parse_union_groups(input);

        assert_eq!(expected_groups, actual_groups);
        assert_eq!(11, sum_counts(&actual_groups));
    }

    //noinspection SpellCheckingInspection
    #[test]
    fn can_parse_intersect_group() {
        let input = "abcx
abcy
abcz";
        let expected_set: HashSet<char> = vec!('a', 'b', 'c').into_iter().collect();

        assert_eq!(
            expected_set,
            intersect_group_from_lines(input)
        );
    }

    #[test]
    fn can_parse_and_count_intersect_groups() {
        let input = "abc

a
b
c

ab
ac

a
a
a
a

b";
        let expected_groups: Vec<HashSet<char>> = vec!(
            vec!('a', 'b', 'c').into_iter().collect(),
            vec!().into_iter().collect(),
            vec!('a').into_iter().collect(),
            vec!('a').into_iter().collect(),
            vec!('b').into_iter().collect(),
        );

        let actual_groups = parse_intersect_groups(input);

        assert_eq!(expected_groups, actual_groups);
        assert_eq!(6, sum_counts(&actual_groups));
    }
}
