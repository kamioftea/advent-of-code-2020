//! This is my solution for [Advent of Code - Day 6](https://adventofcode.com/2020/day/6) -
//! _Custom Customs_
//!
//! Today is themed around set manipulation. It presents two very similar puzzles differing in one
//! word, `anyone` vs `everyone`, but the solutions are different enough that I essentially solved
//! the two parts separately.
//!
//! [`parse_union_groups`] builds the sets for part 1, [`parse_intersect_groups`] builds the sets 
//! for part 2, and [`sum_counts`] reduces each solution set into a single number that can be used
//! as the puzzle answer. The only awkwardness was there isn't an easy implementation of intersect
//! on [`std::collections::HashSet<T>`] (in stable). I presume as it would put an unwanted bound on
//! `T` implementing [`Copy`], so I implemented a simple version [`intersect`].

use std::fs;
use std::collections::HashSet;
use std::hash::Hash;

/// The entry point for running the solutions with the 'real' puzzle input.
///
/// - The puzzle input is expected to be at `<project_root>/res/day-6-input`
/// - It is expected this will be called by [`super::main()`] when the user elects to run day 6.
pub fn run() {
    let contents = fs::read_to_string("res/day-6-input").expect("Failed to read file");

    let union_groups = sum_counts(&parse_union_groups(contents.as_str()));
    println!("Sum of union group counts: {}", union_groups);

    let intersect_groups = sum_counts(&parse_intersect_groups(contents.as_str()));
    println!("Sum of intersect group counts: {}", intersect_groups);
}

/// Parse the puzzle inputs into a set per group that is the union of all the people in that groups'
/// answers.
///
/// > The form asks a series of 26 yes-or-no questions marked `a` through `z`. All you need to do is
/// > identify the questions for which anyone in your group answers "yes". Since your group is just
/// > you, this doesn't take very long.
/// >
/// > Another group asks for your help, then another, and eventually you've collected answers from
/// > every group on the plane (your puzzle input). Each group's answers are separated by a blank
/// > line, and within each group, each person's answers are on a single line.
/// 
/// This only handles splitting the groups by empty lines and delegates to 
/// [`union_group_from_lines`] to build the union for each group.
/// 
/// # Example from Tests
/// ```
/// let input = "abc\n\n\na\nb\nc\n\nab\nac\n\na\na\na\na\n\nb";
/// 
/// let expected_groups: Vec<HashSet<char>> = vec!(
///     vec!('a', 'b', 'c').into_iter().collect(),
///     vec!('a', 'b', 'c').into_iter().collect(),
///     vec!('a', 'b', 'c').into_iter().collect(),
///     vec!('a').into_iter().collect(),
///     vec!('b').into_iter().collect(),
/// );
/// 
/// let actual_groups = parse_union_groups(input);
/// 
/// assert_eq!(expected_groups, actual_groups);
/// assert_eq!(11, sum_counts(&actual_groups));
/// ```
fn parse_union_groups(input: &str) -> Vec<HashSet<char>> {
    input.split("\n\n").into_iter().map(|str| union_group_from_lines(str)).collect()
}

/// Parses a string representing a group and returns the set of questions that were answered yes by
/// __anyone__.
///
/// > For each of the people in their group, you write down the questions for which
/// > they answer "yes", one per line. For example:
/// >
/// > ```text
/// > abcx
/// > abcy
/// > abcz
/// > ```
/// >
/// > In this group, there are 6 questions to which anyone answered "yes": a, b, c, x, y, and z.
/// > (Duplicate answers to the same question don't count extra; each question counts at most once.)
fn union_group_from_lines(lines: &str) -> HashSet<char> {
    let mut group = HashSet::new();

    lines.chars()
        .filter(|chr| ('a'..='z').contains(chr))
        .for_each(|chr| { group.insert(chr); });

    group
}

/// Parse the puzzle inputs into a set per group that is the intersect of all the people in that
/// groups' answers.
///
/// This is the same as [`parse_union_groups`], but it delegates to [`intersect_group_from_lines`].
fn parse_intersect_groups(input: &str) -> Vec<HashSet<char>> {
    input.split("\n\n").into_iter().map(|str| intersect_group_from_lines(str)).collect()
}

/// Intersect two sets returning a new set with only the values present in both `a` and `b`
///
/// # Examples from Tests
/// ```
/// let abc: HashSet<char> = vec!('a', 'b', 'c').into_iter().collect();
/// let ab: HashSet<char> = vec!('a', 'b').into_iter().collect();
/// let abd: HashSet<char> = vec!('a', 'b', 'd').into_iter().collect();
/// let def: HashSet<char> = vec!('d', 'e', 'f').into_iter().collect();
/// let empty: HashSet<char> = vec!().into_iter().collect();
/// assert_eq!(ab, intersect(abc.clone(), ab.clone()));
/// assert_eq!(ab, intersect(abc.clone(), abd.clone()));
/// assert_eq!(empty, intersect(abc.clone(), def.clone()));
/// assert_eq!(empty, intersect(abc.clone(), empty.clone()));
/// assert_eq!(empty, intersect(empty.clone(), ab.clone()));
/// assert_eq!(empty, intersect(empty.clone(), empty.clone()));/
/// ```
fn intersect<T: Hash + Eq + Copy>(a: HashSet<T>, b: HashSet<T>) -> HashSet<T> {
    let mut out = HashSet::new();

    a.iter().filter(|&t| b.contains(t) ).for_each(|&t| {out.insert(t);});

    out
}

/// Parses a string representing a group and returns the set of questions that were answered yes by
/// __everyone__.
///
/// This uses the fact that a single line is still a valid group of one person to reuse the line
/// parsing logic from [`union_group_from_lines`]. Then iterates folds resulting sets into their
/// intersect
///
/// # Example from Tests
/// ```
/// let input = "abc\n\n\na\nb\nc\n\nab\nac\n\na\na\na\na\n\nb";
///
/// let expected_groups: Vec<HashSet<char>> = vec!(
///     vec!('a', 'b', 'c').into_iter().collect(),
///     vec!().into_iter().collect(),
///     vec!('a').into_iter().collect(),
///     vec!('a').into_iter().collect(),
///     vec!('b').into_iter().collect(),
/// );
/// let actual_groups = parse_intersect_groups(input);
///
/// assert_eq!(expected_groups, actual_groups);
/// assert_eq!(6, sum_counts(&actual_groups));
/// ```
fn intersect_group_from_lines(lines: &str) -> HashSet<char> {
    lines.lines()
        .map(|line| union_group_from_lines(line))
        .fold(
            ('a'..='z').into_iter().collect(),
            |acc, answers| intersect(acc, answers)
        )
}

/// Returns the sum of the sizes of the sets of answers for each group.
fn sum_counts(groups: &Vec<HashSet<char>>) -> usize {
    groups.iter().map(|group| group.len()).sum()
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;
    use day_6::{union_group_from_lines, parse_union_groups, sum_counts, intersect_group_from_lines, parse_intersect_groups, intersect};

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

    #[test]
    fn can_intersect() {
        let abc: HashSet<char> = vec!('a', 'b', 'c').into_iter().collect();
        let ab: HashSet<char> = vec!('a', 'b').into_iter().collect();
        let abd: HashSet<char> = vec!('a', 'b', 'd').into_iter().collect();
        let def: HashSet<char> = vec!('d', 'e', 'f').into_iter().collect();
        let empty: HashSet<char> = vec!().into_iter().collect();

        assert_eq!(ab, intersect(abc.clone(), ab.clone()));
        assert_eq!(ab, intersect(abc.clone(), abd.clone()));
        assert_eq!(empty, intersect(abc.clone(), def.clone()));
        assert_eq!(empty, intersect(abc.clone(), empty.clone()));
        assert_eq!(empty, intersect(empty.clone(), ab.clone()));
        assert_eq!(empty, intersect(empty.clone(), empty.clone()));
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
